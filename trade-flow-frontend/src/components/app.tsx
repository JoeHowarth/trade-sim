import React, {useEffect, useState} from "react";
import "react-bulma-components/dist/react-bulma-components.min.css";
import {Box} from "react-bulma-components";
import Graph from "./graph";
import {InfoTable, InfoTableMode} from "./info_table";
import {Api} from "../sim_api";
import {ErrorBoundary} from "./error_boundary";
import {Form, Formik, Field} from "formik";

type AppProps = {
  api: Api;
  initialVisual: RGraph;
};

const App = ({api, initialVisual}: AppProps) => {
  const [tick, setTick] = useState(api.lastModel().tick);
  const [isStarted, setIsStarted] = useState(true);
  const [infoTableMode, setInfoTableMode] = useState(null);
  const [fetchRate, setFetchRate] = useState(1500) // hook this up to an input to allow control

  // control fetching the model
  useEffect(() => {
    (async () => {
      if (isStarted) {
        const nextModel = await api.nextModel(fetchRate);
        setIsStarted(isStarted => {
          if (isStarted) { // check again after fetching. Is it possible for this callback to observe isStarted changing?
            setTick((oldTick: number) => {
              console.assert(
                oldTick < nextModel?.tick,
                "Expected old tick to be < nextModel.tick"
              );
              return nextModel.tick;
            });
          }
          return isStarted
        })
      }
    })()
  }, [tick, isStarted]);

  // @ts-ignore
  return (
    <>
      <div
        style={{
          zIndex: 2,
          position: "absolute",
          width: '100%',
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
              {/*<div className="level-item">*/}
              {/*  <Formik initialValues={3000} onSubmit={v => setFetchRate(v)}>*/}
              {/*    <Form>*/}
              {/*      <Field name="rate" type="text"/>*/}
              {/*    </Form>*/}
              {/*  </Formik>*/}
              {/*</div>*/}
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


export default App;
