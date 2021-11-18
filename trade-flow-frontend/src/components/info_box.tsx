import {isNumber} from "lodash";
import React, {
  FC,
  PropsWithChildren,
  useEffect,
  useRef,
  useState,
} from "react";
import {Table} from "react-bulma-components";
import {dbg} from "../sim_api";

interface PosProps {
  position: Point;
}

export const AtPosition = (
  props: PropsWithChildren<PosProps & { offset?(ref: React.MutableRefObject<any>): Point }>
) => {
  const ref = useRef(null);
  const [offset, setOffset] = useState({x: -10000, y: -10000});
  useEffect(() => {
    if (ref.current) {
      setOffset(props.offset ? props.offset(ref) : {x: 0, y: 0});
    }
  }, [ref.current]);

  return (
    <div
      style={{
        position: "absolute",
        top: props.position.y - offset.y,
        left: props.position.x - offset.x,
      }}
      ref={ref}
    >
      {props.children}
    </div>
  );
};

export const Centered = (props: PropsWithChildren<PosProps>) => {
  return (
    <AtPosition
      offset={(ref) => ({
        x: ref.current.offsetWidth / 2,
        y: ref.current.offsetHeight / 2,
      })}
      {...props}
    />
  );
};

export const CenteredAbove = (props: PropsWithChildren<PosProps>) => {
  return (
    <AtPosition
      offset={(ref) => ({
        x: ref.current.offsetWidth / 2,
        y: ref.current.offsetHeight,
      })}
      {...props}
    />
  );
};

export const AgentInfoTable = (
  props: PropsWithChildren<{ agent: MAgent; oldAgent: MAgent }>
) => {
  const {agent, oldAgent} = props
  return (
    <Table
      style={{zIndex: 1}}
      className="is-narrow is-bordered is-striped is-hoverable"
    >
      <thead>
      <tr>
        <th className="is-size-7">AgentId</th>
        <th className="is-size-7">Cargo</th>
        <th className="is-size-7">Money</th>
        <th className="is-size-7">Profit</th>
        <th className="is-size-7">Prev. City</th>
      </tr>
      </thead>
      <tbody>
      <tr key={agent.id}>
        <th className="is-size-7">{agent.id}</th>
        <td className="is-size-7"> {agent.cargo} </td>
        <td className="is-size-7"><Number>
          {round(agent.money, 0)}
        </Number></td>
        <td className="is-size-7"><Number oldValue={round(oldAgent?.money, 0)}>
          {round(agent.money, 0)}
        </Number></td>
        <td className="is-size-7">{oldAgent?.location}</td>
      </tr>
      </tbody>
    </Table>
  );
};

export const MarketInfoTable = (
  props: PropsWithChildren<{ node: MNode; oldMarkets: Map<Good, MarketInfo> }>
) => {
  console.log('from market info table, oldMarkets: ', props.oldMarkets, props)
  return <Table
    style={{zIndex: 1}}
    className="is-narrow is-bordered is-striped is-hoverable mb-0"
  >
    <thead>
    <tr>
      <th className="is-size-7">Good</th>
      <th className="is-size-7">
        Co<span/>
      </th>
      <th className="is-size-7">Prod</th>
      <th className="is-size-7">Supp</th>
      <th className="is-size-7">Price (delta)</th>
    </tr>
    </thead>
    <tbody>
    {Array.from(props.node.markets, ([good, info]) => (
      <tr key={good}>
        <th className="is-size-7">{good}</th>
        <td>
          <Number>{round(info.consumption, 0)}</Number>
        </td>
        <td>
          <Number>{round(info.production, 0)}</Number>
        </td>
        <td>
          <Number oldValue={props.oldMarkets?.get(good).supply}>
            {round(info.supply, 0)}
          </Number>
        </td>
        <td>
          <Number oldValue={props.oldMarkets?.get(good).price}>
            {info.price}
          </Number>
        </td>
      </tr>
    ))}
    </tbody>
  </Table>
}

export const View = (
  props: PropsWithChildren<PosProps>
) => {
  return (
    <CenteredAbove position={transform(props.position, {x: 0, y: -30})}>
      {props.children}
    </CenteredAbove>
  );
};

function meanPriceAndStdDev(models: Models, goods?: Good[]): { mean: number, stdDev: number } {
  if (!goods) {
    let set = new Set<Good>()
    models[0].nodes.forEach(node => {
      node.markets.forEach((_, good) => set.add(good))
    })
    goods = Array.from(set)
  }
  // TODO use filter to only consider some goods

  return {
    mean: 0,
    stdDev: 0
  }
}

export const Number = (
  props: {
    oldValue?: number;
    precision?: number;
  } & React.PropsWithChildren<{}>
) => {
  if (!isNumber(props.children)) {
    return null;
  }
  const {oldValue, precision} = props;
  const value = round(props.children, precision ? precision : 2);
  return (
    <div>
      <span className="is-size-7">{value} </span>
      {oldValue !== undefined ? (
        <span
          style={{
            fontSize: 9,
            color: value < oldValue ? "rgb(240, 58, 2)" : "rgb(9, 190, 30)",
          }}
        >
          {(value > oldValue ? "+" : "") + round(value - oldValue, 2)}
        </span>
      ) : null}
    </div>
  );
};

export function round(x: number, places: number): number {
  if (places === 0) {
    return Math.round(x);
  }
  return Math.round(x * Math.pow(10, places)) / Math.pow(10, places);
}

export function transform(p: Point, a: Point): Point {
  return {x: p.x + a.x, y: p.y + a.y};
}
