import React, { useCallback, useEffect, useRef, useState } from "react";
import { Api } from "../sim_api";
import App from './app'


export default () => {
  const api = new Api();
  const [hasInitial, setInitial] = useState(false);
  useEffect(() => {
    api.initialState().then((data) => {
      setInitial(data !== undefined);
      console.log("Got initial state", data);
    });
  }, []);

  return hasInitial? (
    <App api={api} />
  ) : (
    <h1>Loading</h1>
  );
};
