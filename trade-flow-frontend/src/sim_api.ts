import * as Random from "./random_gen";
import axios from "axios";
import _ from "lodash";

export class MockApi implements SimApi {
  model: Model;

  async nextState(): Promise<Model> {
    let newModel = _.cloneDeep(this.model);
    newModel.nodes[0].markets.get("Grain").price += Math.random() * 2 - 1;
    return newModel;
  }

  async initialState(): Promise<{ visual: RGraph; model: Model }> {
    const x = Random.GenerateInitial();
    this.model = x.modelInitial;
    return { visual: x.visualInitial, model: x.modelInitial };
  }
}

export class Api implements SimApi {
  static baseUrl: string = "http://127.0.0.1:3030";
  static async getModel(): Promise<Model> {
    const resp = await axios
      .get<Model>(Api.baseUrl + "/state")
      .catch(errorHandler<Model>());
    resp.data.nodes = resp.data.nodes.map((n) => {
      n.markets = new Map(Object.entries(n.markets));
      return n;
    });
    return resp.data;
  }

  async nextState(): Promise<Model> {
    return Api.getModel();
  }

  async initialState(): Promise<{ visual: RGraph; model: Model }> {
    let visualResp = axios
      .get<RGraph>(Api.baseUrl + "/rgraph")
      .catch(errorHandler<RGraph>());
    let model = await Api.getModel();
    console.log("initial model", model)

    let visual = (await visualResp).data;
    visual.edges = visual.edges.map(e => {
      return {
        nodes: e.nodes.map(n => visual.nodes.find(n1 => n1.id === n.id))
      }
    })
    return { visual, model };
  }
}

function errorHandler<T>(): (e) => { data: T; err: any } {
  return (e) => {
    console.error("request failed", e);
    return { data: null, err: e };
  };
}
