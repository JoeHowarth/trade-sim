interface Point {
  x: number;
  y: number;
}

interface RGraph {
  nodes: RNode[]
  edges: REdge[]
}

interface RNode extends Point {
  id: NodeId
  radius: number
  color?: string
}

interface REdge {
  nodes: RNode[]
}

interface RAgent extends Point {
  id: AgentId
}

interface MNode {
  id: NodeId
  markets: Map<Good, MarketInfo>
  links: NodeId[]
}

interface MEdge {
  nodes: NodeId[]
}

interface MAgent {
  id: AgentId
  cargo: Good
  location: NodeId
  money: Money
}

type Money = number
type Good = string
type NodeId = string
type AgentId = string

interface MarketInfo {
  supply: number
  consumption: number
  production: number
  price: number
}

interface Model {
  tick: number
  nodes: MNode[]
  edges: MEdge[]
  agents: MAgent[]
}

interface SimApi {
  nextState(): Promise<Model>

  initialState(): Promise<{ visual: RGraph; model: Model; }>
}