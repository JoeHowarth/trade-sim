import axios from "axios";
import _ from "lodash";

// export class MockApi implements SimApi {
//   private model: Model;
//   private models: Models
//
//   async initialState(): Promise<{ visual: RGraph; model: Model }> {
//     const x = Random.GenerateInitial();
//     this.model = x.modelInitial;
//     return {visual: x.visualInitial, model: x.modelInitial};
//   }
//
//   async getModel(tick?: number): Promise<Model> {
//     let newModel = _.cloneDeep(this.model);
//     newModel.nodes[0].markets.get("Grain").price += Math.random() * 2 - 1;
//     this.models.push(newModel)
//     return newModel;
//   }
//
//   async getModels(): Promise<Models> {
//     return this.models
//   }
// }

export class Api implements SimApi {
  static baseUrl: string = "http://0.0.0.0:3030";
  static visualUrl: string = Api.baseUrl + "/rgraph";
  static modelUrl: string = Api.baseUrl + "/state";

  // models indexed by tick
  private models: Models = [];

  async fetchModel(i: number): Promise<Model | undefined> {
    // only use network if model not already cached
    if (this.models[i] !== undefined) {
      return this.models[i]
    }
    let data = await get<Model>(Api.modelUrl + "/" + i);
    if (data === null) {
      return undefined
    }
    data.agents = new Map(Object.entries(data.agents));
    data.nodes = new Map(Object.entries(data.nodes));
    data.nodes.forEach((n, _) => {
      n.markets = new Map(Object.entries(n.markets));
      return n;
    });
    // cache model
    this.models[data.tick] = data;
    return data;
  }

  getModel(i: number): Model|undefined {
    if (this.models[i] === undefined) {
      console.log("Found missing model. Fetching...", i);
      this.getModel(i);
    }
    return this.models[i]
  }

  getModels(): Models {
    return this.models
  }

  async nextModel(): Promise<Model> {
    let model: Model
    while (model === undefined) {
      // TODO: rate limit using promise based timer
      model = await this.getModel(this.lastModel().tick+1);
    }
    return model;
  }

  async initialState(): Promise<{ visual: RGraph; model: Model }> {
    const visual = Api.getVisual();
    const model = this.getModel(0);
    const ret = { visual: await visual, model: await model };
    console.log("Initial model:", ret.model);
    return ret;
  }

  async fetchModels(): Promise<Models> {
    const last = this.lastModel();
    let promises = []
    for (let i = 0; i < last.tick; ++i) {
      if (this.models[i] === undefined) {
        console.log("Found missing model. Fetching...", i);
        promises.push(this.getModel(i))
      }
    }
    // TODO: look up how to block on all promises cleanly
    for (let promise of promises) {
      await promise
    }
    return this.models;
  }

  lastModel(): Model {
    return this.models[this.models.length - 1];
  }

  private static async getVisual(): Promise<RGraph> {
    let d = await get<WireRGraph>(Api.visualUrl);
    return {
      nodes: new Map(Object.entries(d.nodes)),
      edges: d.edges.map((e) => ({
        nodes: e.nodes.map((n) => d.nodes[n]),
      })),
    };
  }
}

async function get<T>(url: string): Promise<T> {
  return axios
    .get<T>(url)
    .catch(errorHandler<T>())
    .then((r) => r.data);
}

function errorHandler<T>(): (e) => { data: T; err: any } {
  return (e) => {
    console.error("request failed", e);
    return { data: null, err: e };
  };
}
