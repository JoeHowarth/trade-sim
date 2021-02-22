import React, {useCallback, useEffect, useState} from "react";
import {Stage, Layer, Star, Text, Rect, Ring, Circle, Line} from "react-konva";
import Konva from "konva";
import {KonvaEventObject} from "konva/types/Node";

export const VisualAgent = (props: {
  agent: RAgent, onClick?(e: KonvaEventObject<any>): void
}) => {
  const {agent} = props
  return (
    <Ring
      x={agent.x + Math.random() / 5}
      y={agent.y + Math.random() / 5}
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
