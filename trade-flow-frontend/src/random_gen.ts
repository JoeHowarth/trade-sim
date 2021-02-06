export function range(end: number): number[] {
  let a = [];
  for (let i = 0; i < end; i += 1) {
    a.push(i);
  }
  return a;
}

export function GenerateInitial(): { visualInitial: RGraph; modelInitial: Model } {
  console.log("in generateInitial")
  const mnodes = range(3).map((id) => MNode(id.toString()));
  const rnodes = range(3).map((id) => Node(id));
  const medges = Edge(mnodes.map((n, i) => Object.assign(n, rnodes[i])));
  for (let node of mnodes) {
    for (let edge of medges) {
      let idx = edge.nodes.findIndex(id => id === node.id)
      if (idx != -1) {
        node.links.push(edge[(idx + 1) % 2])
      }
    }
  }
  const model: Model = {
    tick: 0,
    nodes: mnodes,
    edges: medges,
    agents: [],
  };
  const redges = medges.map((e) => ({
    nodes: e.nodes.map((id) => rnodes.find((n1) => n1.id === id)),
  }));
  const visual = {nodes: rnodes, edges: redges};
  return {modelInitial: model, visualInitial: visual};
}

export function MarketInfo(): MarketInfo {
  return {
    supply: Math.random() * 1000,
    consumption: Math.random() * 100,
    production: Math.random() * 200,
    price: Math.random() * 20,
  };
}

export function MNode(id: string, goodsList?: string[]): MNode {
  const goods = goodsList ? goodsList : ["Grain", "Timber", "Iron"];
  return {
    id,
    markets: new Map(goods.map((g) => [g, MarketInfo()])),
    links: []
  };
}

export function distance<T extends Point>(a: T, b: T): number {
  return Math.sqrt((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y));
}

export function Edge(nodes: (MNode & RNode)[]): MEdge[] {
  const allEdges: MEdge[] = nodes.flatMap((n) => {
    const closest = nodes
      .filter((n1) => n.id !== n1.id && distance(n, n1) < 1000)
      .sort((a, b) => distance(n, a) - distance(n, b));
    console.log("closest", closest)
    let edges = [0.7, 0.2, 0.2, 0.2, 0.1].flatMap((p, idx) =>
      Math.random() < p && idx < closest.length - 1 ? [closest[idx].id] : []
    );
    edges = edges.length === 0 ? [closest[0].id] : edges
    console.log("edges", edges)
    return edges.map(
      (id) =>
        ({
          nodes: [id, n.id].sort(),
        } as MEdge)
    );
  });
  return allEdges.filter((e1, i, a) => a.indexOf(e1) === i)
}

export function Position(): Point {
  return {
    x: Math.random() * (window.innerWidth - 100) + 50,
    y: Math.random() * (window.innerHeight - 100) + 50,
  };
}

export function Node(id: number): RNode {
  let pt = Position() as RNode;
  pt.id = id + "";
  pt.radius = 50;
  return pt;
}

function randomIndex(len: number): number {
  return Math.floor(Math.random() * len);
}

export function Graph(numNodes: number, numEdges: number): RGraph {
  if (numNodes <= 2) {
    return {nodes: [], edges: []};
  }

  const nodes = Array.from({length: numNodes}).map((n, i) => Node(i));
  const edges = Array.from({length: numEdges})
    .map(() => {
      const fromIdx = randomIndex(nodes.length);

      let toIdx = -1;
      while (toIdx === -1 || toIdx === fromIdx) {
        toIdx = randomIndex(nodes.length);
      }

      return {nodes: [nodes[toIdx], nodes[fromIdx]]} as REdge;
    })
    .filter((e, i, edges) => !edges.find((e1, i1) => i1 !== i && e1 === e));

  return {nodes, edges};
}
