import React, { useCallback, useEffect, useState } from "react";
import ReactDOM from "react-dom";
import "react-bulma-components/dist/react-bulma-components.min.css";
import Bulma, { Box, Column, Columns } from "react-bulma-components";
import Konva, { Stage, Layer } from "react-konva";
import { VisualNode, VisualEdge } from "./visual";
import { KonvaEventObject } from "konva/types/Node";
import { ViewMarketInfo } from "./InfoBox";
import * as Random from "./random_gen";
import _ from 'lodash'

const exampleModel: Model = {
  nodes: [],
  edges: [],
  agents: [],
};


class MockApi implements SimApi {
  nextState(model: Model): Model {
    let newModel = _.cloneDeep(model)
    newModel.nodes[0].markets.get("Grain").price += Math.random() * 2 - 1;
    return newModel
  }
}

const Wrapper = () => {
  return <App {...Random.GenerateInitial()}/>
}

const App = ({visualInitial, modelInitial}: {visualInitial: RGraph, modelInitial: Model}) => {
  const [tick, setTick] = useState(0);
  const [isStarted, setIsStarted] = useState(false)
  const api = new MockApi
  useEffect(() => {
    const newModel = api.nextState(model)
    setModel(newModel)
  }, [tick]);

  useEffect(() => {
    let interval = null;
    if (isStarted) {
      interval = setInterval(() => {
        setTick(tick => tick + 1);
      }, 1000);
    } else if (!isStarted) {
      clearInterval(interval);
    }
    return () => {
      clearInterval(interval);
      console.log("clearing interval")
    }
  }, [isStarted]);

  const [model, setModel] = useState(modelInitial);
  const [modelCopy, setModelCopy] = useState(modelInitial);
  const [oldModel, setOldModel] = useState(null);
  useEffect(() => {
    setOldModel(modelCopy);
    setModelCopy(model);
  }, [model]);
  console.log("model", model);

  return (
    <>
      {/* <div className="columns is-mobile is-tablet">
        <div className="column is-narrow is-narrow-tablet is-narrow-desktop is-narrow-mobile "> */}
      <Box>
        <div className="level is-mobile">
          <div className="level-left">
            <div className="level-item">
              <div className="has-text-centered has-border">Tick: {tick}</div>
            </div>
            <div className="level-item">
      <div onClick={() => setIsStarted(!isStarted)} className="button">{isStarted? "Stop" : "Start"}</div>
            </div>
            <div className="level-item">
              <div className="button">Bye</div>
            </div>
          </div>
        </div>
      </Box>
      {/* </div>
      </div> */}
      <Graph graph={visualInitial} model={model} />
    </>
  );
};

const Graph = (props: { graph: RGraph; model: Model }) => {
  const [graph, setGraph] = useState(props.graph);
  useEffect(() => {
    setGraph(props.graph);
  }, [props.graph]);

  const nodeMap = new Map(
    graph.nodes.map((n) => [
      n.id,
      { visual: n, model: props.model.nodes.find((m) => m.id === n.id) },
    ])
  );

  // info components
  const [nodeClicked, setNodeClicked] = useState(null);

  return (
    <>
      {nodeClicked ? (
        <ViewMarketInfo
          position={nodeMap.get(nodeClicked).visual}
          node={nodeMap.get(nodeClicked).model}
        />
      ) : null}
      <Canvas>
        {graph.nodes.map((n) => (
          <VisualNode
            node={n}
            key={n.id}
            onClick={() => {
              console.log("clicked");
              setNodeClicked(nodeClicked === n.id ? null : n.id);
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
