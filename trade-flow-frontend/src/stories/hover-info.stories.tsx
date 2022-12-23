import React, {  } from "react"
import "bulma/css/bulma.min.css"
import { ComponentMeta } from "@storybook/react"
import { HoverInfo, AgentHoverInfo, NodeHoverInfo } from "./hover-info"
import { mockMAgents, mockMNodes } from "./misc/mocks"

export default {
  title: "Hover Info",
  component: HoverInfo,
} as ComponentMeta<typeof HoverInfo>

export const Main = () => {
  return <HoverInfo position={{ x: 300, y: 400 }} data={[{ height: 1 }]} />
}

export const AgentHover = () => {
  return <AgentHoverInfo position={{ x: 300, y: 400 }} data={mockMAgents[0]} />
}

export const NodeHover = () => {
  return <NodeHoverInfo position={{ x: 300, y: 400 }} data={mockMNodes[0]} />
}
