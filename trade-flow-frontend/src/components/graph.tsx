import React, {useRef, useState} from "react";
import {
  AgentInfoTable,
  MarketInfoTable,
  View,
} from "./info-box";
import {MovingThing, VisualAgent, VisualEdge, VisualNode} from "./visual";
import {KonvaEventObject} from "konva/types/node";
import Canvas from "./canvas";
import {CSS_COLOR_NAMES, useTraceUpdate} from "../utils";

type GraphProps = { graph: RGraph; model: Model; oldModel: Model };

export default (props: GraphProps) => {
  useTraceUpdate(props)
  const {model, oldModel} = props;
  const [graph, setGraph]: [RGraph, any] = useState(props.graph);

  // info components
  const [clickedNodes, setClickedNodes]: [Set<NodeId>, any] = useState(
    new Set<NodeId>(model.nodes.keys())
  );
  const [clickedAgents, setClickedAgents]: [Set<AgentId>, any] = useState(
    new Set<AgentId>()
  );
  const agentFill = useRef<Map<AgentId, string>>(new Map());

  const toggleNodeInfo = (id: NodeId) => {
    if (clickedNodes.has(id)) {
      clickedNodes.delete(id);
    } else {
      clickedNodes.add(id);
    }
    setClickedNodes(new Set(clickedNodes));
  };
  const toggleAgentInfo = (id: AgentId) => {
    if (clickedAgents.has(id)) {
      clickedAgents.delete(id);
    } else {
      clickedAgents.add(id);
    }
    setClickedAgents(new Set(clickedAgents));
  };

  return (
    <>
      {Array.from(clickedNodes.keys()).map((id) => (
        <View position={graph.nodes.get(id)}>
          <h3>{id}</h3>
          <p><b>Agents</b> {Array.from(model.agents.values()).filter(a => a.location === id).map(a => a.id).join(', ')}</p>
          <MarketInfoTable
            key={id}
            node={model.nodes.get(id)}
            oldMarkets={oldModel.nodes.get(id).markets}
          />
        </View>
      ))}
      {Array.from(clickedAgents.keys()).map((id) => {
        const agent = {
          agent: model.agents.get(id),
          oldAgent: oldModel.agents.get(id),
        };
        const node = graph.nodes.get(agent.agent.location);
        return (
          <View position={node}>
            <h3>{id}</h3>
            <AgentInfoTable
              key={id}
              agent={agent.agent}
              oldAgent={agent.oldAgent}
            />
          </View>
        );
      })}
      <Canvas>
        <MovingThing/>
        {Array.from(graph.nodes.values()).map((n) => (
          <VisualNode
            node={n}
            key={n.id}
            price={model.nodes.get(n.id).markets.get("Grain").price}
            onClick={() => {
              console.log("clicked");
              toggleNodeInfo(n.id);
            }}
            onDragEnd={(e: KonvaEventObject<DragEvent>) => {
              const node = graph.nodes.get(n.id);
              node.x = e.target.x();
              node.y = e.target.y();
              setGraph({...graph});
            }}
          />
        ))}
        {graph.edges.map((e, i) => (
          <VisualEdge edge={e} key={i}/>
        ))}
        {Array.from(model.agents, ([id, a]) => {
          let node = graph.nodes.get(a.location);
          return (
            <VisualAgent
              key={id}
              onClick={() => toggleAgentInfo(id)}
              agent={{
                fill: getFill(id, agentFill),
                id: id,
                x: node.x,
                y: node.y,
              }}
            />
          );
        })}
      </Canvas>
    </>
  );
};

function getFill(id: AgentId, fillRef: {current: Map<AgentId, string>}): string {
  if (!fillRef.current.has(id)) {
    const color = CSS_COLOR_NAMES[Math.floor(Math.random() * (CSS_COLOR_NAMES.length -1))]
    fillRef.current.set(id, color)
  }
  return fillRef.current.get(id)
}

function getRandomColor() {
  var letters = '0123456789ABCDEF';
  var color = '#';
  for (var i = 0; i < 6; i++) {
    color += letters[Math.floor(Math.random() * 16)];
  }
  return color;
}