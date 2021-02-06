import React, {useEffect, useRef, useState} from "react";
import "react-bulma-components/dist/react-bulma-components.min.css";
import {Box} from "react-bulma-components";
import Graph from "./graph";

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

  // for use inside control fetching callback
  let tickRef = useRef(model.tick);
  tickRef.current = model.tick;
  const [isStarted, setIsStarted] = useState(true);

  // control fetching model
  useEffect(() => {
    let interval = null;
    if (isStarted) {
      interval = setInterval(() => {
        api.nextState().then((newModel) => {
          if (newModel.tick > tickRef.current) {
            setModel((oldModel) => {
              setOldModel(oldModel);
              return newModel;
            });
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

  return (
    <>
      <Box>
        <div className="level is-mobile">
          <div className="level-left">
            <div className="level-item">
              <div className="has-text-centered has-border">
                Tick: {model.tick}
              </div>
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
      <Graph graph={initial.visual} model={model} oldModel={oldModel}/>
    </>
  );
};

export default App;
