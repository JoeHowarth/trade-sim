interface Point {
  x: number;
  y: number;
}

interface RGraph {
  nodes: RNode[]
  edges: REdge[]
}

interface RNode extends Point{
  id: NodeId
  radius: number
  color?: string
}

interface REdge {
  nodes: RNode[]
}

interface RAgent {
  edge: REdge
  id: AgentId
}

interface MNode {
  id: NodeId
  markets: Map<Good, MarketInfo>
}

interface MEdge {
  nodes: [NodeId]
}

interface MAgent {
  id: AgentId
  location: NodeId | MEdge
}

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
  nodes: MNode[]
  edges: MEdge[]
  agents: MAgent[]
}

interface SimApi {
  nextState(model: Model): Model
}