import React, { useCallback, useEffect, useRef, useState } from "react";
import { Api } from "../sim_api";
import App from './app'


export default () => {
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
