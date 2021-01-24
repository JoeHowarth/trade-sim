import React, { useCallback, useEffect, useRef, useState } from "react";
import ReactDOM from "react-dom";
import "react-bulma-components/dist/react-bulma-components.min.css";
import Bulma, { Box, Column, Columns } from "react-bulma-components";
import Konva, { Stage, Layer } from "react-konva";
import { VisualNode, VisualEdge } from "./visual";
import { KonvaEventObject } from "konva/types/Node";
import { ViewMarketInfo } from "./InfoBox";
import _ from "lodash";
import { MockApi, Api } from "../sim_api";

const exampleModel: Model = {
  tick: 0,
  nodes: [],
  edges: [],
  agents: [],
};

const Wrapper = () => {
  console.log("top of wrapper");
  const api = new Api();
  const [initial, setInitial] = useState(null);
  useEffect(() => {
    api.initialState().then((data) => {
      setInitial(data);
      console.log("Got initial state", data);
    });
  }, []);

  return initial == null ? (
    <h1>Loading</h1>
  ) : (
    <App initial={initial} api={api} />
  );
};

const App = ({
  initial,
  api,
}: {
  api: SimApi;
  initial: {
    visual: RGraph;
    model: Model;
  };
}) => {
  // maintain oldModel
  const [model, setModel] = useState(initial.model);
  // const [modelCopy, setModelCopy] = useState(initial.model);
  const [oldModel, setOldModel] = useState(initial.model);
  // useEffect(() => {
  //   setOldModel(modelCopy);
  //   setModelCopy(model);
  // }, [model]);
  console.log("model", model);
  let tickRef = useRef(model.tick)
  tickRef.current = model.tick

  // const [tick, setTick] = useState(0);
  const [isStarted, setIsStarted] = useState(false);

  // control fetching model
  useEffect(() => {
    let interval = null;
    if (isStarted) {
      interval = setInterval(() => {
        api.nextState().then((newModel) => {
          if (newModel.tick > tickRef.current) {
            setModel(oldModel => {
              setOldModel(oldModel)
              return newModel
            });
            console.log("new model:", newModel);
          }
        });
      }, 1000);
    }
    return () => {
      if (interval !== null) {
        clearInterval(interval);
        console.log("clearing interval");
      }
    };
  }, [isStarted]);
  console.log("oldModel, newModel ticks", oldModel.tick, model.tick)

  return (
    <>
      <Box>
        <div className="level is-mobile">
          <div className="level-left">
            <div className="level-item">
              <div className="has-text-centered has-border">Tick: {model.tick}</div>
            </div>
            <div className="level-item">
              <div onClick={() => setIsStarted(!isStarted)} className="button">
                {isStarted ? "Stop" : "Start"}
              </div>
            </div>
            <div className="level-item">
              <div className="button">Bye</div>
            </div>
          </div>
        </div>
      </Box>
      <Graph graph={initial.visual} model={model} oldModel={oldModel} />
    </>
  );
};

const Graph = (props: { graph: RGraph; model: Model; oldModel: Model }) => {
  const [graph, setGraph] = useState(props.graph);
  useEffect(() => {
    setGraph(props.graph);
  }, [props.graph]);

  const nodeMap = new Map<
    NodeId,
    { visual: RNode; model: MNode; oldModel: MNode }
  >(
    graph.nodes.map((n) => [
      n.id,
      {
        visual: n,
        model: props.model.nodes.find((m) => m.id === n.id),
        oldModel: props.oldModel.nodes.find((m) => m.id === n.id),
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

const Canvas = (props) => {
  const { children } = props;
  return (
    <Stage
      style={{ position: "absolute", top: 0, right: 0, zIndex: -1 }}
      width={window.innerWidth}
      height={window.innerHeight}
      {...props}
    >
      <Layer>{children}</Layer>
    </Stage>
  );
};

export default Wrapper;
