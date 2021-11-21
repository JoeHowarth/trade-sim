import React, { useEffect, useState } from "react";
import { Api } from "../sim_api";
import App from './app'

export default () => {
  const api = new Api();
  const [initialVisual, setInitialVisual] = useState(null);
  useEffect(() => {
    api.initialState().then((data) => {
      setInitialVisual(data?.visual);
      console.log("Got initial state", data);
    });
  }, []);

  if  (initialVisual) {
    console.log("models from wrapper", api.getModels())
    console.log("initialVisual", initialVisual)
    return <App api={api} initialVisual={initialVisual} />
  } else {
    console.log("bye")
    return <h1>Loading</h1>
  }
};
