import React, { useCallback, useEffect, useRef, useState } from "react";
import { ViewMarketInfo } from "./InfoBox";
import { VisualNode, VisualEdge } from "./visual";
import { KonvaEventObject } from "konva/types/node";
import Canvas from "./canvas";

type GraphProps = { graph: RGraph; model: Model; oldModel: Model };
type NodeMap = Map<NodeId, { visual: RNode; model: MNode; oldModel: MNode }>;

export default (props: GraphProps) => {
  const {model, oldModel} = props
  const [graph, setGraph] = useState(props.graph);

  const nodeMap: NodeMap = new Map(
    graph.nodes.map((n) => [
      n.id,
      {
        visual: n,
        model: model.nodes.find((m) => m.id === n.id),
        oldModel: oldModel.nodes.find((m) => m.id === n.id),
      },
    ])
  );

  // info components
  const [clickedNodes, setClickedNodes] = useState(new Set<NodeId>());

  const toggleInfo = (id: NodeId) => {
    if (clickedNodes.has(id)) {
      clickedNodes.delete(id);
    } else {
      clickedNodes.add(id);
    }
    setClickedNodes(new Set(clickedNodes));
  };

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
      <Canvas>
        {graph.nodes.map((n) => (
          <VisualNode
            node={n}
            key={n.id}
            onClick={() => {
              console.log("clicked");
              toggleInfo(n.id);
            }}
            onDragEnd={(e: KonvaEventObject<DragEvent>) => {
              const node = graph.nodes.find((node) => node.id === n.id);
              node.x = e.target.x();
              node.y = e.target.y();
              setGraph({ ...graph });
            }}
          />
        ))}
        {graph.edges.map((e, i) => (
          <VisualEdge edge={e} key={i} />
        ))}
      </Canvas>
    </>
  );
};
