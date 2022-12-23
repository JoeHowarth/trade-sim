import React, { Dispatch, SetStateAction, useState } from "react"
import "bulma/css/bulma.min.css"
import { ComponentMeta } from "@storybook/react"
import { CanvasWithOverlay } from "../canvas"
import { Circle } from "react-konva"
import { View } from "./graphics-utils"
import { dbg, scale, transform, UseStateType } from "../../utils"
import { VisualAgent, VisualNode, VisualEdge } from "./graphics"
import { KonvaEventObject } from "konva/types/node"

export default {
  title: "Graphics",
  component: View,
} as ComponentMeta<typeof View>


const Template = (args: {
  main: (args: { posState: UseStateType<Point> }) => JSX.Element
}) => {
  const [position, setPosition] = React.useState({ x: 300, y: 300 })

  return (
    <CanvasWithOverlay
      children={args.main({ posState: [position, setPosition] })}
      OverlayDom={
        <View height={20} position={position}>
          HI
        </View>
      }
      border={true}
    />
  )
}

export const TestCircle = Template.bind({})
TestCircle.args = {
  main: ({ posState: [{ x, y }, setPos] }) => (
    <Circle
      fill={"red"}
      radius={10}
      draggable
      onDragMove={e =>
        setPos({ x: e.target.position().x, y: e.target.position().y })
      }
      x={x}
      y={y}
    ></Circle>
  ),
}

export const Agent = Template.bind({})
Agent.args = {
  main: ({ posState: [{ x, y }, setPos] }) => (
    <VisualAgent
      agent={{
        fill: "red",
        id: "bob",
        x,
        y,
      }}
      onClick={() => setPos(({ x, y }) => ({ x: x + 10, y: y + 50 }))}
      animateSec={5}
    ></VisualAgent>
  ),
}

export const Node = Template.bind({})
Node.args = {
  main: ({ posState: [{ x, y }, setPos] }) => (
    <VisualNode
      onClick={() => setPos(({ x, y }) => ({ x: x + 10, y: y + 50 }))}
      node={{
        x,
        y,
        id: "paris",
      }}
      price={20}
      onDragMove={e =>
        setPos({ x: e.target.position().x, y: e.target.position().y })
      }
    ></VisualNode>
  ),
}

export const EdgeTemplate = () => {
  const [node1, setNode1] = useState({ x: 300, y: 200, id: "paris" })
  const [node2, setNode2] = useState({ x: 200, y: 400, id: "toulouse" })

  return (
    <CanvasWithOverlay
      children={
        <>
          <VisualNode
            onClick={() => {}}
            node={node1}
            price={20}
            onDragMove={e =>
              setNode1(node => ({
                ...node,
                x: e.target.position().x,
                y: e.target.position().y,
              }))
            }
          />
          <VisualNode
            onClick={() => {}}
            node={node2}
            price={40}
            onDragMove={e =>
              setNode2(node => ({
                ...node,
                x: e.target.position().x,
                y: e.target.position().y,
              }))
            }
          />
          <VisualEdge
            edge={{
              nodes: [node1, node2],
            }}
          />
        </>
      }
      OverlayDom={
        <View
          height={10}
          position={scale(transform(node1, node2), { x: 0.5, y: 0.5 })}
        >
          HI
        </View>
      }
      border={true}
    />
  )
}
