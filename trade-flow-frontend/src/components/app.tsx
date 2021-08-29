import React, {useEffect, useRef, useState} from "react";
import "react-bulma-components/dist/react-bulma-components.min.css";
import {Box} from "react-bulma-components";
import Graph from "./graph";
import {InfoTable} from "./info_table";

type AppProps = {
  api: SimApi;
  initial: {
    visual: RGraph;
    model: Model;
  };
};

const App = ({initial, api}: AppProps) => {
  // maintain oldModel
  const [model, setModel] = useState(initial.model);
  const [oldModel, setOldModel] = useState(initial.model);

  // for use inside control-fetching callback
  let tickRef = useRef(model.tick);
  tickRef.current = model.tick;
  const [isStarted, setIsStarted] = useState(true);

  // control fetching the model
  useEffect(() => {
    let interval = null;
    if (isStarted) {
      interval = setInterval(async () => {
        const newModel = await api.getModel()
        if (newModel === undefined) {
          console.warn("not expecting undefined model")
          return
        }
        if (newModel.tick > tickRef.current) {
          setModel((oldModel) => {
            setOldModel(oldModel);
            return newModel;
          });
        }
      }, 1000);
    }
    return () => {
      if (interval !== null) {
        clearInterval(interval);
        console.log("clearing interval");
      }
    };
  }, [isStarted]);

  const [cityTableVisible, setCityTableVisible] = useState(false)

  return (
    <>
      <div
        style={{
          zIndex: 2,
          position: "absolute",
          top: 0,
          left: 0,
        }}>
        <Box>
          <div className="level is-mobile">
            <div className="level-left">
              <div className="level-item">
                <div className="has-text-centered has-border">
                  Tick: {model.tick}
                </div>
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
                  onClick={() => setCityTableVisible(!cityTableVisible)}
                  className="button"
                >
                  City Table
                </div>
              </div>
            </div>
          </div>
        </Box>

        {
          cityTableVisible ?
            <Box style={{
              zIndex: 2,
              margin: 20,
              width: 300,
              border: '1px solid rgba(0, 0, 0, 0.05)',
              maxWidth: '50%'
            }} onClick={() => console.warn("I've been clicked")}>
              <InfoTable model={model} oldModel={oldModel}/>
            </Box>
            : null
        }
      </div>
      <Graph graph={initial.visual} model={model} oldModel={oldModel}/>
    </>
  )
    ;
};


export default App;
