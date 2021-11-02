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

  async getModel(i: number): Promise<Model | undefined> {
    let data = await get<Model>(Api.modelUrl + "/" + i);
    data.agents = new Map(Object.entries(data.agents));
    data.nodes = new Map(Object.entries(data.nodes));
    data.nodes.forEach((n, _) => {
      n.markets = new Map(Object.entries(n.markets));
      return n;
    });
    return data;
  }

  async nextState(): Promise<Model> {
    const model = await this.getModel(this.lastModel().tick+1);
    // TODO: if model == undefined (i.e. 404), retry in a loop until success
    if (model.tick > this.models[this.models.length - 1].tick) {
      this.models[model.tick] = model;
    }
    return model;
  }

  async initialState(): Promise<{ visual: RGraph; model: Model }> {
    const visual = Api.getVisual();
    const model = this.getModel();
    const ret = { visual: await visual, model: await model };
    console.log("Initial model:", ret.model);
    return ret;
  }

  async getModels(): Promise<Models> {
    const last = this.lastModel();
    for (let i = 0; i < last.tick; ++i) {
      if (this.models[i] === undefined) {
        console.log("Found missing model. Fetching...", i);
        this.models[i] = await this.getState(i);
      }
    }
    return this.models;
  }

  async getState(tick: number): Promise<Model> {
    return this.getModel(tick);
  }

  private lastModel(): Model {
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
