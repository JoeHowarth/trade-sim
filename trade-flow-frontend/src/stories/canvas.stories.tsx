import React from "react";
import 'bulma/css/bulma.min.css'
import { ComponentStory, ComponentMeta } from "@storybook/react";
import Canvas, { CanvasWithOverlay } from "./canvas";
import { Circle } from "react-konva";
import { Button } from "react-bulma-components";

export default {
  title: "Canvas",
  component: CanvasWithOverlay,
} as ComponentMeta<typeof Canvas>;

export const Template = (args) => (
  <CanvasWithOverlay OverlayDom={args.OverlayDom} children={args.children} />
);

export const Main = Template.bind({});
Main.args = {
  children: (
    <Circle
      radius={100}
      fill="red"
      x={100}
      y={100}
      draggable
      onClick={(e) => console.log("clicked", e)}
    />
  ),
  OverlayDom: () => (
    <div>
      <Button>"Click me!</Button>
      <p style={{ width: "fit-content", border: "1px solid black" }}>
        A bunch of text to make the window longer
      </p>
    </div>
  ),
};
