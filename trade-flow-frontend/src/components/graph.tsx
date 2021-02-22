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
        model: model.nodes.find((m) => m.id === n.id),
        oldModel: oldModel.nodes.find((m) => m.id === n.id),
      },
    ])
  )
  const agentMap: AgentMap = new Map(
    model.agents.map(a => [a.id, {
      agent: a,
      oldAgent: oldModel.agents.find(a2 => a2.id === a.id)
    }])
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
        const agent = agentMap.get(id);
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
        {model.agents.map((a, i) => {
          let node = nodeMap.get(a.location).visual
          return <VisualAgent
            key={a.id}
            onClick={() => toggleAgentInfo(a.id)}
            agent={{
              id: a.id,
              x: node.x,
              y: node.y,
            }}/>
        })}
      </Canvas>
    </>
  )
}
