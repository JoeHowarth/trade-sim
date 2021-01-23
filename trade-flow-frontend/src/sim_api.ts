import * as Random from "./random_gen";
import axios from "axios";
import _ from 'lodash'

export class MockApi implements SimApi {
  async nextState(model: Model): Promise<Model> {
    let newModel = _.cloneDeep(model);
    newModel.nodes[0].markets.get("Grain").price += Math.random() * 2 - 1;
    return newModel;
  }

  async initialState(): Promise<{ visual: RGraph; model: Model }> {
    console.log("in generateInitial");
    const x = Random.GenerateInitial();
    return { visual: x.visualInitial, model: x.modelInitial };
  }
}

function errorHandler<T>(): (e) => { data: T; err: any } {
  return (e) => {
    console.error("request failed", e);
    return { data: null, err: e };
  };
}

export class Api implements SimApi {
  async initialState(): Promise<{ visual: RGraph; model: Model }> {
    let modelResp = axios
      .get<Model>("http://127.0.0.1:3030/model")
      .catch(errorHandler<Model>());
    let visualResp = axios
      .get<RGraph>("http://127.0.0.1:3030/rgraph")
      .catch(errorHandler<RGraph>());

    let model = (await modelResp).data;
    let visual = (await visualResp).data;
    return { visual, model };
  }

  async nextState(model: Model): Promise<Model> {
    const resp = await axios
      .get<Model>("http://127.0.0.1:3030/model")
      .catch(errorHandler<Model>());
    return resp.data;
  }
}
