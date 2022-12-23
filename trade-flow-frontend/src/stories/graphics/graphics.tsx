import { KonvaEventObject } from "konva/types/Node"
import React, { useState } from "react"
import { Circle, Line, Ring } from "react-konva"
import { transform } from "../../utils"

export const VisualAgent = (props: {
  agent: RAgent
  onClick?(e: KonvaEventObject<MouseEvent>): void
  animateSec?: number
}) => {
  const { agent } = props
  const [pos, setPos] = useState<Point>(agent)
  const [node, setNode] = useState(null)
  const [offset, setOffset] = useState({
    x: Math.random() * 8 - 4,
    y: Math.random() * 8 - 4,
  })
  if (node) {
    console.log("animate agent")
    node.to({
      ...transform(agent, offset),
      duration: props.animateSec || 1,
      onFinish: () => setPos(agent),
    })
  }
  return (
    <Ring
      ref={node => setNode(node)}
      {...transform(pos, offset)}
      innerRadius={4}
      outerRadius={7}
      fill={props.agent.fill ? props.agent.fill : "red"}
      onClick={props.onClick}
    />
  )
}

export const VisualEdge = ({ edge }: { edge: REdge }) => {
  return (
    <Line
      strokeWidth={2}
      stroke="gray"
      points={[
        edge.nodes[0].x,
        edge.nodes[0].y,
        edge.nodes[1].x,
        edge.nodes[1].y,
      ]}
    />
  )
}

export const VisualNode = (props: {
  node: RNode
  price: number
  onClick
  onDragMove(e: KonvaEventObject<DragEvent>): void
}) => {
  return (
    <Circle
      draggable
      {...props}
      onDragMove={props.onDragMove}
      key={props.node.id}
      x={props.node.x}
      y={props.node.y}
      radius={Math.max(props.price / 6, 2)}
      fill={props.node.color || "grey"}
    />
  )
}

export const MovingThing = () => {
  const [node, setNode] = useState(null)
  // console.log(node)
  return (
    <Circle
      radius={5}
      ref={n => setNode(n)}
      x={500}
      y={500}
      fill="red"
      onClick={() => {
        node.to({
          x: Math.random() * 200 + 200,
          y: Math.random() * 200 + 200,
        })
      }}
    />
  )
}
