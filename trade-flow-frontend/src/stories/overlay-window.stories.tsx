import React, { Children, useState } from "react";
import "bulma/css/bulma.min.css";
import { ComponentStory, ComponentMeta } from "@storybook/react";
import OverlayWindow from "./overlay-window";
import { Circle } from "react-konva";
import { Button } from "react-bulma-components";
import { CanvasWithOverlay } from "./canvas";
import BasicTable from "./basic-table";
import { createColumnHelper } from "@tanstack/react-table";

export default {
  title: "OverlayWindow",
  component: OverlayWindow,
} as ComponentMeta<typeof OverlayWindow>;

export const Template = (args) => {
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
          {args.children}
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

export const Main = Template.bind({});
Main.args = {
  title: "A thing",
  children: (
    <div>
      <Button color={"success"}>Click me!</Button>
      <p style={{ border: "1px solid black" }}>
        A bunch of text to make the window longer
      </p>
    </div>
  ),
};

export const Table = Template.bind({});
type CityGood = {
  name: string;
  price: number;
  amount: number;
};
const columnHelper = createColumnHelper<CityGood>();
Table.args = {
  title: "City goods",
  children: (
    <BasicTable
      columns={[
        columnHelper.accessor("name", { header: "Name" }),
        columnHelper.accessor("price", {
          header: "Price",
          cell: (i) => i.getValue().toString(),
        }),
        columnHelper.accessor("amount", { header: "Amount" }),
      ]}
      defaultData={[
        {name: "Wheat", price: 23, amount: 1520},
        {name: "Wood", price: 50, amount: 250}
      ]}
    />
  ),
};
export const TableNoColumns = Template.bind({});
TableNoColumns.args = {
  title: "City goods",
  children: (
    <BasicTable
      defaultData={[
        {name: "Wheat", price: 23, amount: 1520},
        {name: "Wood", price: 50, amount: 250}
      ]}
    />
  ),
};
