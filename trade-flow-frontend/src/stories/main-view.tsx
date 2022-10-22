import { flatMapDeep } from "lodash";
import React, { useMemo, useState } from "react";
import BasicTable from "./basic-table";
import OverlayWindow from "./overlay-window";
import TopControlBar from "./top-control-bar";

const DEFAULT_TICK_PER_SECOND = 1;

enum View {
  Agents = "Agents",
  Cities = "Cities",
}

function MainView({
  agents,
  nodes,
}: {
  agents: MAgent[];
  nodes: MNode[];
}): JSX.Element {
  const [activeView, setActiveView] = useState<View | null>(View.Agents);
  const [ticksPerSecond, setTicksPerSecond] = useState(DEFAULT_TICK_PER_SECOND);
  const [isPlaying, setIsPlaying] = useState(true);

  // const views = useMemo(() => {
  //   const citiesView = nodes.map((n) => {
  //     const markets = Object.fromEntries(
  //       Array.from(n.markets.entries()).map(([good, info]) => {
  //         return [good, info.price];
  //       })
  //     );
  //     return { ...markets, city: n.id };
  //   });
  //   return {
  //     [View.Agents]: () => BasicTable({ defaultData: [...agents] }),
  //     [View.Cities]: () => BasicTable({ defaultData: citiesView }),
  //   }
  // }, [nodes,agents]);
  const citiesView = nodes.map((n) => {
    const markets = Object.fromEntries(
      Array.from(n.markets.entries()).map(([good, info]) => {
        return [good, info.price];
      })
    );
    return { ...markets, city: n.id };
  });
  const views = {
    [View.Agents]: () => <BasicTable defaultData={[...agents]} />,
    [View.Cities]: () => <BasicTable defaultData={citiesView} />,
  };

  return (
    <>
      <TopControlBar
        title="Trade Sim"
        onClickExit={() => console.log("Exit clicked")}
        tick={0}
        isPaused={!isPlaying}
        togglePlay={() => setIsPlaying((x) => !x)}
        faster={() => setTicksPerSecond((t) => t * 1.5)}
        slower={() => setTicksPerSecond((t) => t / 1.5)}
        setActiveView={(clicked) =>
          setActiveView((active) => (clicked === active ? null : clicked))
        }
        views={Object.values(View)}
      />
      {views[activeView] ? (
        <OverlayWindow title={activeView} onClickExit={() => {}}>
          {views[activeView]()}
        </OverlayWindow>
      ) : null}
    </>
  );
}

export default MainView;

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
