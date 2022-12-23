import React, { Children, useState } from "react"
import "bulma/css/bulma.min.css"
import { ComponentStory, ComponentMeta } from "@storybook/react"
import OverlayWindow from "./overlay-window"
import { Circle } from "react-konva"
import { Button } from "react-bulma-components"
import { CanvasWithOverlay } from "../canvas"
import BasicTable from "../misc/basic-table"
import { createColumnHelper } from "@tanstack/react-table"
import { AgentsTab } from "./agents"
import { mockMAgents, mockMNodes, mockModels } from "../misc/mocks"
import { CitiesTab } from "./cities"

export default {
  title: "Tabs",
  component: OverlayWindow,
} as ComponentMeta<typeof OverlayWindow>

const Template = args => {
  const [clicked, setClicked] = useState(false)
  return (
    <CanvasWithOverlay
      domStyle={{
        top: 20,
        left: 20,
      }}
      OverlayDom={[
        <OverlayWindow
          title={args.title}
          onClickExit={() => setClicked(x => !x)}
        >
          {args.children}
        </OverlayWindow>,
      ]}
      children={[
        <Circle
          radius={100}
          fill={clicked ? "blue" : "red"}
          x={100}
          y={100}
          draggable
          onClick={e => console.log("clicked", e)}
        />,
      ]}
    />
  )
}

export const Main = Template.bind({})
Main.args = {
  title: "A thing",
  children: (
    <div>
      <Button color={"success"}>Click me!</Button>
      <p style={{ border: "1px solid black" }}>
        A bunch of text to make the window longer
      </p>
    </div>
  ),
}

export const Agents = AgentsTab.bind({})
Agents.args = {
  setActiveWindow: () => {},
  agents: mockMAgents,
  getModels: () => mockModels
}

export const Cities = CitiesTab.bind({})
Cities.args = {
  setActiveView() {},
  nodes: mockMNodes,
  getModels: () => mockModels
}
