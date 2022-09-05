use types::{
    agent::{Agent, Cargo},
    market::{
        exchanger::{Exchanger, MarketInfo},
        Market, Money,
    },
    prelude::*,
    City, CityHandle,
};

pub struct Failed<T>(pub T);

pub fn transition_order(
    order: &Order,
    market: &mut dyn Exchanger, // TODO: change to Market to support multiple goods
    agent_money: &mut Money,
    agent_cargo: &mut Cargo,
) -> Result<()> {
    if agent_cargo.amt > 0 {
        return Err(anyhow!(
            "Cannot buy goods when cargo is non-empty, {:?} {:?}",
            agent_cargo,
            order
        ));
    }
    market.buy(agent_money, order.amt).map(|_| {
         agent_cargo.good = order.good;
         agent_cargo.amt = order.amt.max(0) as u32;
     }).context("Order failed because market 'buy' failed. Usually due to agent not having enough money")
}

pub fn clear_orders(
    mut markets: Query<&mut MarketInfo, With<City>>,
    mut agents: Query<(&mut Money, &mut Cargo), With<Agent>>,
    mut orders: EventReader<Order>,
    mut failed_orders: EventWriter<Failed<Order>>,
) -> Result<()> {
    for order in orders.iter() {
        let mut market = markets
            .get_mut(order.market.entity)
            .expect("market entity not in markets query");
        let (mut wallet, mut cargo) = agents
            .get_mut(order.agent.entity)
            .expect("agent entity not in agents query");
        if let Err(e) = transition_order(
            order,
            &mut market,
            &mut wallet,
            &mut cargo,
        ) {
            failed_orders.send(Failed(order.clone()));
            return Err(e);
        }
    }
    Ok(())
}

mod tests {
    use super::*;
    use crate::setup_tests;
    use types::{
        agent::AgentHandle, market::pricer::LinearPricer, Good,
    };

    #[test]
    fn test_clear_orders() {
        let mut app = setup_tests();
        fn setup(mut cmds: Commands, mut orders: EventWriter<Order>) {
            let grain = Good::from("Grain");
            let ch = {
                let city = City {
                    name: Ustr::from("a_city"),
                };
                let entity = cmds
                    .spawn_bundle((
                        city,
                        MarketInfo {
                            consumption: 15.,
                            supply: 100.,
                            production: 15.,
                            pricer: LinearPricer::new(
                                100., 10., -0.1,
                            ),
                        },
                    ))
                    .id();
                CityHandle { entity, city }
            };
            let ah = {
                let agent = Agent {
                    name: Ustr::from("1_agent"),
                };
                let entity = cmds
                    .spawn_bundle((
                        agent,
                        Money(100.),
                        Cargo {
                            good: grain,
                            amt: 1,
                        },
                    ))
                    .id();
                AgentHandle { agent, entity }
            };

            orders.send_batch(
                vec![
                    Order {
                        good: grain,
                        market: ch,
                        agent: ah,
                        amt: -1,
                    },
                    Order {
                        good: grain,
                        market: ch,
                        agent: ah,
                        amt: 1,
                    },
                    Order {
                        good: grain,
                        market: ch,
                        agent: ah,
                        amt: 400,
                    },
                ]
                .into_iter(),
            )
        }
        app.add_startup_system(setup.system()).add_system(
            clear_orders.system().chain(
                (|markets: Query<(Entity, &MarketInfo, &City)>,
                  agents: Query<(Entity, &Money, &Cargo, &Agent)>,
                  _failed_orders: EventReader<Failed<Order>>| {
                    let city = City {
                        name: Ustr::from("a_city"),
                    };
                    let (_c_e, market, _): (Entity, &MarketInfo, _) = markets
                        .iter()
                        .filter(|(_, _, &c)| c == city)
                        .next()
                        .unwrap();
                    assert_eq!(market.current_price().0, 10.);

                    let agent = Agent {
                        name: Ustr::from("1_agent"),
                    };
                    let (_a_e, agent_money, agent_cargo, _): (Entity, &Money, &Cargo, _) = agents
                        .iter()
                        .filter(|(_, _, _, a)| **a == agent)
                        .next()
                        .unwrap();
                    assert_eq!(agent_money.0, 100.);
                    assert_eq!(agent_cargo.good, Good::from("Grain"));

                    // TODO: finish testing
                    // let failed_order = failed_orders.iter().next();
                    // assert!(failed_order.is_some());
                    // assert_eq!(failed_order.unwrap().amt, 100);
                })
                .system(),
            ),
        );
        app.run()
    }
}
