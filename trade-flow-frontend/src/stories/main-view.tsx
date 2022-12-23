import React, { useState } from "react"
import { dbg } from "../utils"
import BasicTable from "./misc/basic-table"
import { AgentsTab } from "./tabs/agents"
import { CitiesTab } from "./tabs/cities"
import OverlayWindow from "./tabs/overlay-window"
import TopControlBar from "./top-control-bar"

const DEFAULT_TICK_PER_SECOND = 1

export enum TabEnum {
  Agents = "Agents",
  Cities = "Cities",
}

function MainView({
  isPlaying,
  setIsPlaying,
  setTickRate,
  tick,
  agents,
  nodes,
}: {
  isPlaying: boolean
  setIsPlaying: (x: (isPlaying: boolean) => boolean) => void
  setTickRate: (x: (tickRate: number) => number) => void
  tick: number
  agents: MAgent[]
  nodes: MNode[]
}): JSX.Element {
  const [activeView, setActiveView] = useState<TabEnum | null>(TabEnum.Agents)

  return (
    <>
      <TopControlBar
        title="Trade Sim"
        tick={tick}
        isPaused={isPlaying}
        togglePlay={() => setIsPlaying(x => !x)}
        faster={() => setTickRate(t => t / 1.5)}
        slower={() => setTickRate(t => t * 1.5)}
        setActiveView={clicked =>
          setActiveView(active => (clicked === active ? null : clicked))
        }
        views={Object.values(TabEnum)}
      />
      {selectActiveTab(activeView, setActiveView, agents, nodes)}
    </>
  )
}

function selectActiveTab(
  activeView,
  setActiveView,
  agents,
  nodes
): JSX.Element | null {
  switch (activeView) {
    case TabEnum.Agents:
      return <AgentsTab setActiveView={setActiveView} agents={agents} />
    case TabEnum.Cities:
      return <CitiesTab setActiveView={setActiveView} nodes={nodes} />
    case null:
      return null
    default:
      console.warn("Unexpected active tab value")
      return null
  }
}

export default MainView

// class MapIter<A, B> implements IterableIterator<B> {
//   private done = false;
//   constructor(readonly f: (A) => B, readonly input: Iterator<A>) {}
//   static from<A, B>(f: (a: A) => B, input: Iterator<A>): MapIter<A, B> {
//     return new MapIter<A, B>(f, input);
//   }
//   [Symbol.iterator]() {
//     return this;
//   }

//   next() {
//     if (this.done) {
//       return { done: true, value: undefined };
//     }
//     const { done, value } = this.input.next();
//     if (done) {
//       this.done = true;
//       return { done: true, value: undefined };
//     }
//     return { done: false, value: this.f(value) };
//   }
// }

// function map<A, B>(input: Iterator<A>, f: (a: A) => B): IterableIterator<B> {
//   return MapIter.from(f, input);
// }
