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
let models: Models = []

export function dbg<T>(x: T): T {
  console.log(x)
  return x
}

export class Api implements SimApi {
  static baseUrl: string = "http://0.0.0.0:3030";
  static visualUrl: string = Api.baseUrl + "/rgraph";
  static modelUrl: string = Api.baseUrl + "/state";

  // models indexed by tick
  // models: Models = [];
  called = 0

  async fetchModel(i: number): Promise<Model | undefined> {
    this.called++
    // only use network if model not already cached
    if (models[i] !== undefined) {
      return models[i]
    }
    let data = await get<Model>(Api.modelUrl + "/" + i);
    if (data === null) {
      console.debug("fetchModel got null data", i, data)
      return undefined
    }
    data.agents = new Map(Object.entries(data.agents));
    data.nodes = new Map(Object.entries(data.nodes));
    data.nodes.forEach((n, _) => {
      n.markets = new Map(Object.entries(n.markets));
      return n;
    });
    // cache model
    models[data.tick] = data;
    return data;
  }

  getModel(i: number): Model|undefined {
    if (i >= 0 && models[i] === undefined) {
      console.log("Found missing model. Fetching...", i);
      this.fetchModel(i);
    }
    return models[i]
  }

  getModels(): Models {
    return models
  }

  async nextModel(fetchRate?: number): Promise<Model> {
    if (!fetchRate) {
      fetchRate = 3000
    }
    let model: Model
    console.log("[nextModel]  new nextModel call")
    while (model === undefined) {
      model = await this.fetchModel(this.lastModel().tick+1);
      if (model !== undefined) {
        console.log("[nextModel]  found model")
      }
      await new Promise(r => setTimeout(r, fetchRate))
    }
    return model;
  }

  async initialState(): Promise<{ visual: RGraph; model: Model }> {
    const visual = Api.getVisual();
    const model = this.fetchModel(0);
    const ret = { visual: await visual, model: await model };
    console.log("Initial model:", ret.model);
    return ret;
  }

  async fetchModels(): Promise<Models> {
    const last = this.lastModel();
    let promises = []
    for (let i = 0; i < last.tick; ++i) {
      if (models[i] === undefined) {
        console.log("Found missing model. Fetching...", i);
        promises.push(this.getModel(i))
      }
    }
    // TODO: look up how to block on all promises cleanly
    for (let promise of promises) {
      await promise
    }
    return models;
  }

  lastModel(): Model {
    return models[models.length - 1];
  }

  private static async getVisual(): Promise<RGraph> {
    let d = await get<WireRGraph>(Api.visualUrl);
    // console.log("wire rgraph", d)
    const nodes = new Map(Object.entries(d.nodes))
    return {
      nodes,
      edges: d.edges.map((e) => ({
        nodes: e.nodes.map((n) => nodes.get((n))),
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
