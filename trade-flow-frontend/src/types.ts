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

type Models = Model[]

interface Model {
  tick: number
  nodes: Map<NodeId, MNode>
  edges: MEdge[]
  agents: Map<AgentId, MAgent>
}

interface SimApi {
  // if tick is not provided, attempt to fetch the next model
  getModel(tick?: number): Promise<Model>
  getModels(): Promise<Models>
  initialState(): Promise<{ visual: RGraph; model: Model; }>
}