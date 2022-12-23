import React, { useEffect, useRef, useState } from "react"
import "bulma/css/bulma.min.css"
// import "react-bulma-components/dist/react-bulma-components.min.css";
import { Box } from "react-bulma-components"
import { InfoTable, InfoTableMode } from "./info-table"
import { Api } from "../sim_api"
import { ErrorBoundary } from "./error-boundary"
import { Form, Formik, Field } from "formik"
import { CanvasWithOverlay } from "../stories/canvas"
import MainView from "../stories/main-view"
import { Circle } from "react-konva"
import { MovingThing, VisualEdge } from "../stories/graphics"
import { Graph } from "../stories/graph"
import _ from "lodash"

type AppProps = {
  api: Api
  initialVisual: RGraph
}

const App = ({ api, initialVisual }: AppProps) => {
  // const [targetTick, setTargetTick] = useState(api.lastModel().tick);
  const [isStarted, setIsStarted] = useState(true)
  const [tickRate, setTickRate] = useState(1000)
  const [model, setModel] = useState<undefined | Model>(undefined)
  const visualState = useState<RGraph>(initialVisual)
  const [clickedAgents, setClickedAgents] = useState({ s: new Set<AgentId>() })
  const [clickedNodes, setClickedNodes] = useState({ s: new Set<NodeId>() })

  // control the desired tick
  useEffect(() => {
    let shouldSet = true
    if (!isStarted) {
      return
    }
    // fire off the request early
    const modelPromise = api.fetchModel((model?.tick || 0) + 1, true)

    setTimeout(async () => {
      if (shouldSet && (await modelPromise)) {
        setModel(await modelPromise)
      }
    }, tickRate)
    return () => {
      shouldSet = false // "cancel" the timout if a dependency changes
    }
  }, [isStarted, model, tickRate])

  return model?.tick >= 1 ? (
    <CanvasWithOverlay
      domStyle={{ padding: 20 }}
      OverlayDom={
        model
          ? [
              <MainView
                key="main"
                isPlaying={isStarted}
                setIsPlaying={setIsStarted}
                setTickRate={setTickRate}
                tick={model.tick}
                agents={Array.from(model.agents.values())}
                nodes={Array.from(model.nodes.values())}
              ></MainView>,

            ]
          : ["Loading..."]
      }
      children={
        <Graph
          visualState={visualState}
          model={model}
          agentClicked={id => {
            clickedAgents.s.has(id)
              ? clickedAgents.s.delete(id)
              : clickedAgents.s.add(id)
            setClickedAgents({ s: clickedAgents.s })
          }}
          nodeClicked={id => {
            clickedNodes.s.has(id)
              ? clickedNodes.s.delete(id)
              : clickedNodes.s.add(id)
            setClickedNodes({ s: clickedNodes.s })
          }}
        />
      }
    />
  ) : null
}

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

export default App
