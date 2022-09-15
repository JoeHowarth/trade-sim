import React, { useEffect, useState } from "react";
import "bulma/css/bulma.min.css";
import { ComponentMeta } from "@storybook/react";
import BasicTable from "./basic-table";
import { ColumnDef, createColumnHelper } from "@tanstack/react-table";

export default {
  title: "InfoBox",
  component: BasicTable,
} as ComponentMeta<typeof BasicTable>;

const Template = BasicTable;

const defaultData: Person[] = [
  {
    firstName: "Tanner",
    lastName: "linsley",
    age: 24,
    visits: 100,
    status: "In Relationship",
    progress: 50,
  },
  {
    firstName: "Tandy",
    lastName: "miller",
    age: 40,
    visits: 40,
    status: "Single",
    progress: 80,
  },
  {
    firstName: "Joe",
    lastName: "dirte",
    age: 45,
    visits: 20,
    status: "Complicated",
    progress: 10,
  },
];

const columnHelper = createColumnHelper<Person>();

const columns: ColumnDef<Person>[] = [
  columnHelper.accessor("firstName", {
    // cell: (i) => i.getValue(),
    header: "First Name",
  }),
  columnHelper.accessor((row) => row.lastName, {
    id: "lastName",
    // cell: (info) => <i>{info.getValue()}</i>,
    header: () => <span>Last Name</span>,
  }),
  columnHelper.accessor("age", {
    header: () => "Age",
    // cell: (info) => info.renderValue(),
  }),
  columnHelper.accessor("visits", {
    header: () => <span>Visits</span>,
  }),
  columnHelper.accessor("status", {
    header: "Status",
  }),
  columnHelper.accessor("progress", {
    header: "Profile Progress",
    // footer: (info) => info.column.id
  }),
];

export const Primary = Template.bind({});
Primary.args = {
  columns,
  defaultData,
};

export const ChangingProps = (args) => {
  const [i, setI] = useState(0);
  useEffect(() => {
    setInterval(() => setI((i) => i + 1), 500);
  }, []);
  const data = defaultData.map(r => {
    return {...r, age: i}
  })
  return <BasicTable defaultData={data} columns={columns}></BasicTable>
};

type Person = {
  firstName: string;
  lastName: string;
  age: number;
  visits: number;
  status: string;
  progress: number;
};
