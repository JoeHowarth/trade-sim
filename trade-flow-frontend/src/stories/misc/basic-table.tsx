import { Table } from "react-bulma-components"
import * as React from "react"
import {
  ColumnDef,
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  getSortedRowModel,
  SortingState,
  useReactTable,
} from "@tanstack/react-table"

// TODO: If element width changes quickly, table jerks around visually
// Ideas: track max width seen

function BasicTable<RData>({
  columns,
  defaultData,
  tableStyleProps,
  sizeClassName,
  noHeader,
}: {
  columns?: ColumnDef<RData, number | string>[]
  defaultData: RData[]
  tableStyleProps?: {
    size?: "fullwidth" | "narrow"
    striped?: boolean
    bordered?: boolean
    hoverable?: boolean
  }
  sizeClassName?: string
  noHeader?: boolean
}) {
  const [data, setData] = React.useState([...defaultData])
  React.useEffect(() => setData([...defaultData]), [defaultData])
  const [sorting, setSorting] = React.useState<SortingState>([])

  // derive column information with sane defaults if not present
  let _columns: ColumnDef<RData, number | string>[] = columns
  if (!columns && data.length > 0) {
    // @ts-ignore
    // _columns = React.useMemo(() => {
    //   return Object.keys(data[0]).map(key => ({header: capitalize(key), accessorKey: key}))
    // }, [defaultData, columns])
    _columns = Object.keys(data[0]).map(key => ({
      header: capitalize(key),
      accessorKey: key,
    }))
    // console.log("derived", _columns);
  }

  const table = useReactTable({
    data,
    columns: _columns,
    state: {
      sorting,
    },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
  })

  return (
    <Table {...tableStyleProps} hoverable={true}>
      {noHeader ? null : (
        <thead>
          {table.getHeaderGroups().map(headerGroup => (
            <tr key={headerGroup.id}>
              {headerGroup.headers.map(header => (
                <th key={header.id} className={sizeClassName}>
                  <div
                    {...{
                      className: header.column.getCanSort()
                        ? "cursor-pointer select-none"
                        : "",
                      onClick: header.column.getToggleSortingHandler(),
                    }}
                  >
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                    {{
                      asc: " 🔼",
                      desc: " 🔽",
                    }[header.column.getIsSorted() as string] ?? null}
                  </div>
                </th>
              ))}
            </tr>
          ))}
        </thead>
      )}
      <tbody>
        {table.getRowModel().rows.map(row => (
          <tr key={row.id}>
            {row.getVisibleCells().map(cell => (
              <td
                className={
                  (typeof cell.getValue() === "number"
                    ? "has-text-right"
                    : "") +
                  " " +
                  sizeClassName
                }
                key={cell.id}
              >
                {flexRender(cell.column.columnDef.cell, cell.getContext())}
              </td>
            ))}
          </tr>
        ))}
      </tbody>
      {table
        .getFooterGroups()
        // only render footer if it is defined
        .some(fg =>
          fg.headers.some(h => !h.isPlaceholder && h.column.columnDef.footer)
        ) ? (
        <tfoot>
          {table.getFooterGroups().map(footerGroup => (
            <tr key={footerGroup.id}>
              {footerGroup.headers.map(header => (
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
      ) : null}
    </Table>
  )
}

export function col<T>(): (name: keyof T) => any {
}

export const capitalizeFirstHeader = { header: capitalizeFirstHeaderFn }

export function capitalizeFirstHeaderFn(h: any): string {
  return capitalize(h.column.id)
}

export function capitalize(s: string): string {
  if (s.length == 0) {
    return s
  }
  return s[0].toUpperCase() + s.slice(1)
}

export default BasicTable
