import React, { useEffect, useState } from "react";
import "bulma/css/bulma.min.css";
// import "react-bulma-components/dist/react-bulma-components.min.css";
import { Box } from "react-bulma-components";
import Graph from "./graph";
import { InfoTable, InfoTableMode } from "./info-table";
import { Api } from "../sim_api";
import { ErrorBoundary } from "./error-boundary";
import { Form, Formik, Field } from "formik";
import { CanvasWithOverlay } from "../stories/canvas";
import MainView from "../stories/main-view";
import { Circle } from "react-konva";

type AppProps = {
  api: Api;
  initialVisual: RGraph;
};

const App = ({ api, initialVisual }: AppProps) => {
  // const [targetTick, setTargetTick] = useState(api.lastModel().tick);
  const [isStarted, setIsStarted] = useState(true);
  const [tickRate, setTickRate] = useState(1000);
  const [model, setModel] = useState<undefined | Model>(undefined);

  // control the desired tick
  useEffect(() => {
    let shouldSet = true;
    if (!isStarted) {
      return;
    }
    // fire off the request early
    const modelPromise = api.fetchModel((model?.tick || 0) + 1, true);

    setTimeout(async () => {
      if (shouldSet && (await modelPromise)) {
        setModel(await modelPromise);
      }
    }, tickRate);
    return () => {
      shouldSet = false; // "cancel" the timout if a dependency changes
    };
  }, [isStarted, model, tickRate]);

  return (
    <CanvasWithOverlay
      domStyle={{ padding: 20 }}
      OverlayDom={[
        model ? (
          <MainView
            isPlaying={isStarted}
            setIsPlaying={setIsStarted}
            setTickRate={setTickRate}
            tick={model.tick}
            agents={Array.from(model.agents.values())}
            nodes={Array.from(model.nodes.values())}
          ></MainView>
        ) : (
          "Loading..."
        ),
      ]}
      children={[
        <Circle
          radius={100}
          fill={"red"}
          x={100}
          y={100}
          draggable
          onClick={(e) => console.log("clicked", e)}
        />,
      ]}
    />
  );
};

/*
  // @ts-ignore
  return (
    <>
      <div
        style={{
          zIndex: 2,
          position: "absolute",
          width: "100%",
          top: 0,
          left: 0,
        }}
      >
        <Box>
          <div className="level is-mobile">
            <div className="level-left">
              <div className="level-item">
                <div className="has-text-centered has-border">Tick: {tick}</div>
              </div>
              <div className="level-item">
                <div
                  onClick={() => setIsStarted(!isStarted)}
                  className="button"
                >
                  {isStarted ? "Stop" : "Start"}
                </div>
              </div>
              <div className="level-item">
                <div
                  onClick={() =>
                    setInfoTableMode((mode) =>
                      mode == InfoTableMode.cities ? null : InfoTableMode.cities
                    )
                  }
                  className="button"
                >
                  City Table
                </div>
              </div>
              <div className="level-item">
                <div
                  onClick={() =>
                    setInfoTableMode((mode) =>
                      mode == InfoTableMode.agents ? null : InfoTableMode.agents
                    )
                  }
                  className="button"
                >
                  Agent Table
                </div>
              </div>
            </div>
          </div>
        </Box>

        <ErrorBoundary>
          {infoTableMode ? (
            <Box
              style={{
                zIndex: 2,
                margin: 20,
                width: 400,
                border: "1px solid rgba(0, 0, 0, 0.05)",
                maxWidth: "50%",
              }}
              onClick={() => console.warn("I've been clicked")}
            >
              <InfoTable
                mode={infoTableMode}
                model={api.getModel(tick)}
                oldModel={api.getModel(tick - 1)}
              />
            </Box>
          ) : null}
        </ErrorBoundary>
      </div>
      <Graph
        graph={initialVisual}
        model={api.getModel(tick)}
        oldModel={api.getModel(tick - 1)}
      />
    </>
  );
};
*/

export default App;
