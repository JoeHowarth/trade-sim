import { Table } from "react-bulma-components";
import * as React from "react";

import {
  ColumnDef,
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";

function BasicTable<RData>({
  columns,
  defaultData,
}: {
  columns?: ColumnDef<RData, number | string>[];
  defaultData: RData[];
}) {
  const data = defaultData;

  // derive column information with sane defaults if not present
  let _columns: ColumnDef<RData, number | string>[] = columns;
  if (!columns && data.length > 0) {
    // @ts-ignore
    // _columns = React.useMemo(() => {
    //   return Object.keys(data[0]).map(key => ({header: capitalize(key), accessorKey: key}))
    // }, [defaultData, columns])
    _columns = Object.keys(data[0]).map((key) => ({
      header: capitalize(key),
      accessorKey: key,
    }));
    console.log("derived", _columns);
  }
  // const [data] = React.useState(() => [...defaultData]);
  // const rerender = React.useReducer(() => ({}), {})[1];

  const table = useReactTable({
    data,
    columns: _columns,
    getCoreRowModel: getCoreRowModel(),
  });

  return (
    <Table>
      <thead>
        {table.getHeaderGroups().map((headerGroup) => (
          <tr key={headerGroup.id}>
            {headerGroup.headers.map((header) => (
              <th key={header.id}>
                {header.isPlaceholder
                  ? null
                  : flexRender(
                      header.column.columnDef.header,
                      header.getContext()
                    )}
              </th>
            ))}
          </tr>
        ))}
      </thead>
      <tbody>
        {table.getRowModel().rows.map((row) => (
          <tr key={row.id}>
            {row.getVisibleCells().map((cell) => (
              <td key={cell.id}>
                {flexRender(cell.column.columnDef.cell, cell.getContext())}
              </td>
            ))}
          </tr>
        ))}
      </tbody>
      <tfoot>
        {table.getFooterGroups().map((footerGroup) => (
          <tr key={footerGroup.id}>
            {footerGroup.headers.map((header) => (
              <th key={header.id}>
                {header.isPlaceholder
                  ? null
                  : flexRender(
                      header.column.columnDef.footer,
                      header.getContext()
                    )}
              </th>
            ))}
          </tr>
        ))}
      </tfoot>
    </Table>
  );
}

function capitalize(s: string): string {
  if (s.length == 0) {
    return s;
  }
  return s[0].toUpperCase() + s.slice(1);
}

export default BasicTable;