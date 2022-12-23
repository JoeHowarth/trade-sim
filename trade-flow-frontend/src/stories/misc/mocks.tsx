import _ from "lodash"

const rome = "rome"
const paris = "paris"
const cities = [rome, paris]

export const mockRNodes: RNode[] = [
  { x: 100, y: 200, id: rome },
  { x: 300, y: 400, id: paris },
]

export const mockMNodes: MNode[] = [
  {
    id: rome,
    markets: new Map([
      ["Grain", { supply: 20, consumption: 10, production: 10, price: 52.24 }],
      [
        "Potatoes",
        { supply: 20, consumption: 10, production: 10, price: 23.49 },
      ],
      ["Iron", { supply: 20, consumption: 10, production: 10, price: 23.49 }],
    ]),
    links: [paris],
  },
  {
    id: paris,
    markets: new Map([
      ["Grain", { supply: 50, consumption: 20, production: 10, price: 82.15 }],
    ]),
    links: [rome],
  },
]

export const mockRGraph: RGraph = {
  nodes: new Map(mockRNodes.map(n => [n.id, n])),
  edges: [{ nodes: mockRNodes.slice(0, 2) }],
}

export const mockMAgents: MAgent[] = [
  { id: "Charlie", cargo: "Grain", location: paris, money: 27.2 },
  { id: "Bob", cargo: "Grain", location: rome, money: 2 },
]

export const mockModel: Model = {
  tick: 2,
  nodes: new Map(mockMNodes.map(n => [n.id, n])),
  edges: [{ nodes: [mockMNodes[1].id, mockMNodes[0].id] }],
  agents: new Map(mockMAgents.map(a => [a.id, a])),
}

export const mockModels: Model[] = [
  {
    tick: 0,
    nodes: new Map(mockMNodes.map(n => [n.id, n])),
    edges: [{ nodes: [mockMNodes[1].id, mockMNodes[0].id] }],
    agents: new Map(mockMAgents.map(a => [a.id, a])),
  },
  {
    tick: 1,
    nodes: new Map(mockMNodes.map(n => [n.id, updateMarkets(n)])),
    edges: [{ nodes: [mockMNodes[1].id, mockMNodes[0].id] }],
    agents: new Map(mockMAgents.map(a => [a.id, updateAgent(a)])),
  },
  {
    tick: 2,
    nodes: new Map(mockMNodes.map(n => [n.id, updateMarkets(n)])),
    edges: [{ nodes: [mockMNodes[1].id, mockMNodes[0].id] }],
    agents: new Map(mockMAgents.map(a => [a.id, updateAgent(a)])),
  },
]

function updateMarkets<T extends { markets: Map<Good, MarketInfo> }>(obj: T) {
  const newObj = _.cloneDeep(obj)
  for (const [good, market] of newObj.markets.entries()) {
    const oldSupply = market.supply
    market.supply += market.production - market.consumption
    market.price += (oldSupply - market.supply) / 10
  }
  return newObj
}

function updateAgent(agent: MAgent): MAgent {
  const newAgent = _.cloneDeep(agent)
  newAgent.location = cities[Math.floor(Math.random() * cities.length)]
  newAgent.money += Math.random() * 10 - 5
  return newAgent
}
