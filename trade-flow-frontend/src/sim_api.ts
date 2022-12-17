import axios from "axios";
import _ from "lodash";
import {
  createGrpcWebTransport,
  createPromiseClient,
} from "@bufbuild/connect-web";
import { ModelServer } from "../gen/modelserver_connectweb";
import { Model as PbModel, RGraph as PbRGraph } from "../gen/modelserver_pb";
import { setConsoleOptions } from "@storybook/addon-console";
import { sleep } from "./utils";

const transport = createGrpcWebTransport({
  baseUrl: "http://127.0.0.1:50051",
});
const client = createPromiseClient(ModelServer, transport);

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
let models: Models = [];
let agentFills: Map<AgentId, string> = new Map();

export function dbg<T>(x: T, message?: string): T {
  if (message) {
    console.log("[Debug] ", message);
    console.log(x);
  } else {
    console.log("[Debug] ", x);
  }
  return x;
}

function modelFromPb(pb: PbModel): Model {
  return {
    tick: Number(pb.tick),
    agents: new Map(Object.entries(pb.agents)),
    nodes: new Map(
      Object.entries(pb.nodes).map(([id, node]) => [
        id,
        { ...node, markets: new Map(Object.entries(node.markets)) },
      ])
    ),
    edges: pb.edges.map((e) => ({ nodes: [e.from, e.to] })),
  };
}

export class Api {
  async fetchModel(
    i: number,
    retry = false,
    checkCache = true
  ): Promise<Model | undefined> {
    if (checkCache && models[i]) {
      return models[i];
    }
    let model: Model | undefined = undefined;
    do {
      model = await client
        .getModel({ tick: BigInt(i) })
        .then(modelFromPb)
        .catch((e) => {
          console.warn("Error getting model", e);
          return undefined;
        });
      await sleep(100);
    } while (!model && retry);
    models[model.tick] = model;
    return model;
  }

  getModel(i: number): Model | undefined {
    if (i >= 0 && models[i] === undefined) {
      console.log("Found missing model. Fetching...", i);
      this.fetchModel(i);
    }
    return models[i];
  }

  getModels(): Models {
    return models;
  }

  async initialState(): Promise<{ visual: RGraph; model: Model }> {
    const visual = Api.getVisual();
    const model = this.fetchModel(0);
    const ret = { visual: await visual, model: await model };
    await this.fetchModel(0, true);
    console.log("Initial model:", ret.model);
    return ret;
  }

  async fetchModels(): Promise<Models> {
    const last = this.lastModel();
    let promises = [];
    for (let i = 0; i < last.tick; ++i) {
      if (models[i] === undefined) {
        console.log("Found missing model. Fetching...", i);
        promises.push(this.getModel(i));
      }
    }
    // TODO: look up how to block on all promises cleanly
    for (let promise of promises) {
      await promise;
    }
    return models;
  }

  lastModel(): Model {
    return models[models.length - 1];
  }

  private static async getVisual(): Promise<RGraph> {
    const rgraph: PbRGraph = await client.getVisual({});
    const nodes = new Map(Object.entries(rgraph.nodes));
    return {
      nodes,
      edges: rgraph.edges.map((e) => ({
        nodes: [nodes.get(e.to), nodes.get(e.from)],
      })),
    };
  }
}
