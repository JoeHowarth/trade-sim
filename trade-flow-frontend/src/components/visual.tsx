import React, {useCallback, useEffect, useRef, useState} from "react";
import {Stage, Layer, Star, Text, Rect, Ring, Circle, Line, KonvaNodeComponent} from "react-konva";
import {KonvaEventObject} from "konva/types/node";

export const MovingThing = () => {
  const [node, setNode] = useState(null)
  console.log(node)
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

export const VisualAgent = (props: {
  agent: RAgent, onClick?(e: KonvaEventObject<any>): void
}) => {
  const {agent} = props
  const [pos, setPos] = useState<Point>(agent)
  const [node, setNode] = useState(null)
  return (
    <Ring
      ref={node => setNode(node)}
      x={pos.x +  Math.random()}
      y={pos.y + Math.random()}
      onClick={() => node.to({
        x: 1000,
        y: 600,
      })}
      innerRadius={3}
      outerRadius={6}
      fill="red"
      {...props}
    />
  )
}

export const VisualEdge = ({edge}: { edge: REdge }) => {
  return (
    <Line
      strokeWidth={2}
      stroke="gray"
      points={[edge.nodes[0].x, edge.nodes[0].y, edge.nodes[1].x, edge.nodes[1].y]}
    />
  );
};

export const VisualNode = (props: {
  node: RNode;
  onClick,
  onDragEnd(e: KonvaEventObject<DragEvent>): void;
}) => {
  return (
    <Circle
      draggable
      {...props}
      onDragMove={props.onDragEnd}
      key={props.node.id}
      x={props.node.x}
      y={props.node.y}
      radius={20}
      fill="gray"
    />
  );
};
