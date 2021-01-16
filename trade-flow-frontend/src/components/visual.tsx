import React, { useCallback, useEffect, useState } from "react";
import ReactDOM from "react-dom";
import { Container } from "react-bulma-components";
import { Stage, Layer, Star, Text, Rect, Circle, Line } from "react-konva";
import Konva from "konva";
import { KonvaEventListener, KonvaEventObject } from "konva/types/Node";
import { KonvaNodeEvent } from "konva/types/types";
import { ViewMarketInfo } from "./InfoBox";

export const VisualEdge = ({ edge }: { edge: REdge }) => {
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
