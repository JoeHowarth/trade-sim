import React, {useState} from "react"
import {ViewAgentInfo, ViewMarketInfo} from "./info_box"
import {VisualAgent, VisualEdge, VisualNode} from "./visual"
import {KonvaEventObject} from "konva/types/node"
import Canvas from "./canvas"

type GraphProps = { graph: RGraph, model: Model, oldModel: Model }
type NodeMap = Map<NodeId, { visual: RNode, model: MNode, oldModel: MNode }>
type AgentMap = Map<AgentId, { agent: MAgent, oldAgent: MAgent }>

export default (props: GraphProps) => {
  const {model, oldModel} = props
  const [graph, setGraph] = useState(props.graph)

  const nodeMap: NodeMap = new Map(
    graph.nodes.map((n) => [
      n.id,
      {
        visual: n,
        model: model.nodes.get(n.id),
        oldModel: oldModel.nodes.get(n.id)
      },
    ])
  )

  // info components
  const [clickedNodes, setClickedNodes] = useState(new Set<NodeId>())
  const [clickedAgents, setClickedAgents] = useState(new Set<AgentId>())

  const toggleNodeInfo = (id: NodeId) => {
    if (clickedNodes.has(id)) {
      clickedNodes.delete(id)
    } else {
      clickedNodes.add(id)
    }
    setClickedNodes(new Set(clickedNodes))
  }
  const toggleAgentInfo = (id: AgentId) => {
    if (clickedAgents.has(id)) {
      clickedAgents.delete(id)
    } else {
      clickedAgents.add(id)
    }
    setClickedAgents(new Set(clickedAgents))
  }

  return (
    <>
      {Array.from(clickedNodes.keys()).map((id) => (
        <ViewMarketInfo
          key={id}
          position={nodeMap.get(id).visual}
          node={nodeMap.get(id).model}
          oldMarkets={nodeMap.get(id).oldModel.markets}
        />
      ))}
      {Array.from(clickedAgents.keys()).map(id => {
        const agent = {agent: model.agents.get(id), oldAgent: oldModel.agents.get(id)};
        const node = nodeMap.get(agent.agent.location).visual
        return <ViewAgentInfo
          key={id}
          position={node}
          agent={agent.agent}
          oldAgent={agent.oldAgent}
        />
      })}
      <Canvas>
        {graph.nodes.map((n) => (
          <VisualNode
            node={n}
            key={n.id}
            onClick={() => {
              console.log("clicked")
              toggleNodeInfo(n.id)
            }}
            onDragEnd={(e: KonvaEventObject<DragEvent>) => {
              const node = graph.nodes.find((node) => node.id === n.id)
              node.x = e.target.x()
              node.y = e.target.y()
              setGraph({...graph})
            }}
          />
        ))}
        {graph.edges.map((e, i) => (
          <VisualEdge edge={e} key={i}/>
        ))}
        {Array.from(model.agents, ([id, a]) => {
          let node = nodeMap.get(a.location).visual
          return <VisualAgent
            key={id}
            onClick={() => toggleAgentInfo(id)}
            agent={{
              id: id,
              x: node.x,
              y: node.y,
            }}/>
        })}
      </Canvas>
    </>
  )
}
