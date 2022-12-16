import React, { useEffect, useState } from "react";
import "bulma/css/bulma.min.css";
import { ComponentMeta } from "@storybook/react";
import TopControlBar from "./top-control-bar";
import MainView from "./main-view";
import { CanvasWithOverlay } from "./canvas";
import OverlayWindow from "./overlay-window";
import { Circle } from "react-konva";

export default {
  title: "MainView",
  component: MainView,
} as ComponentMeta<typeof MainView>;

const marketInfo: MarketInfo = {
  price: 200,
  consumption: 2,
  production: 2,
  supply: 2,
};

export const Main = MainView.bind({});
const args: Parameters<typeof MainView>[0] = {
  agents: [{ id: "Bob", cargo: "Wheat", location: "Berlin", money: 120 }],
  nodes: [
    { id: "Berlin", links: [], markets: new Map([["Wheat", marketInfo]]) },
    {
      id: "Saint Petersberg",
      links: [],
      markets: new Map([["Wheat", { ...marketInfo, price: 1 }]]),
    },
  ],
};
Main.args = args;

export const Template2 = (args) => {
  const [clicked, setClicked] = useState(false);
  return (
    <CanvasWithOverlay
      domStyle={{
        top: 20,
        left: 20,
      }}
      OverlayDom={() => (
        <OverlayWindow
          title={args.title}
          onClickExit={() => setClicked((x) => !x)}
        >
          <MainView {...args}></MainView>
        </OverlayWindow>
      )}
      children={[
        <Circle
          radius={100}
          fill={clicked ? "blue" : "red"}
          x={100}
          y={100}
          draggable
          onClick={(e) => console.log("clicked", e)}
        />,
      ]}
    />
  );
};
Template2.args = args