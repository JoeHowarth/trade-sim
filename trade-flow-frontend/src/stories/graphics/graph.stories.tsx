import React, { useRef, useState } from "react"
import "bulma/css/bulma.min.css"
import { ComponentMeta } from "@storybook/react"
import { CanvasWithOverlay } from "../canvas"
import { View } from "./graphics-utils"
import { Graph } from "./graph"
import { mockModel, mockRGraph } from "../misc/mocks"

export default {
  title: "Graph",
  component: Graph,
} as ComponentMeta<typeof Graph>

const Template = () => {
  const visualState = useState(mockRGraph)
  const [model, setModel] = useState(mockModel)
  const clickedAgents = useRef(new Set<AgentId>())
  const clickedNodes = useRef(new Set<NodeId>())
  const [_, trigger] = useState(0)

  const agentHovers = Array.from(clickedAgents.current.keys()).map(id => {
    const agent = model.agents.get(id)
    return (
      <View key={agent.id} position={visualState[0].nodes.get(agent.location)}>
        `{agent.id} {agent.money}`
      </View>
    )
  })

  const nodeHovers = Array.from(clickedNodes.current.keys()).map(id => {
    const node = model.nodes.get(id)
    return (
      <View key={node.id} position={visualState[0].nodes.get(id)}>
        `{node.id}`
      </View>
    )
  })

  return (
    <CanvasWithOverlay
      children={
        <Graph
          visualState={visualState}
          model={model}
          agentClicked={id => {
            clickedAgents.current.has(id)
              ? clickedAgents.current.delete(id)
              : clickedAgents.current.add(id)
            trigger(x => x + 1)
          }}
          nodeClicked={id => {
            clickedNodes.current.has(id)
              ? clickedNodes.current.delete(id)
              : clickedNodes.current.add(id)
            trigger(x => x + 1)
          }}
        />
      }
      OverlayDom={
        <>
          {agentHovers}
          {nodeHovers}
        </>
        // <View height={20} position={position}>
        //   HI
        // </View>
      }
    />
  )
}

export const Main = Template.bind({})
