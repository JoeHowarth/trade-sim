import React, { useEffect, useState } from "react";
import "bulma/css/bulma.min.css";
import { ComponentMeta } from "@storybook/react";
import TopControlBar from "./top-control-bar";
import MainView from "./main-view";

export default {
  title: "MainView",
  component: MainView,
} as ComponentMeta<typeof MainView>;

export const Main = MainView.bind({});
const args: Parameters<typeof MainView>[0] = {
  agents: [],
  nodes: [],
};
Main.args = args;
