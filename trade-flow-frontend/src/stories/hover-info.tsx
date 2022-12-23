import { ColumnDef, createColumnHelper } from "@tanstack/react-table"
import React from "react"
import { Box } from "react-bulma-components"
import BasicTable, { capitalize, capitalizeFirstHeader } from "./misc/basic-table"
import { View } from "./graphics/graphics-utils"

export function HoverInfo<RData>({
  position,
  data,
  columns,
  extraTableProps,
  title,
  footer,
}: {
  position: Point
  data: RData[]
  columns?: ColumnDef<RData, number | string>[]
  extraTableProps?: any
  title?: JSX.Element
  footer?: JSX.Element
}) {
  return (
    <View position={position}>
      <Box className="p-2" style={{ pointerEvents: "auto" }}>
        {title}
        <BasicTable {...extraTableProps} defaultData={data} columns={columns} />
        {footer}
      </Box>
    </View>
  )
}

export function AgentHoverInfo({
  position,
  data,
}: {
  position: Point
  data: MAgent
}) {
  const helper = createColumnHelper<MAgent>()
  const columns = [
    helper.accessor("id", {
      header: h => h.column.id.toUpperCase(), // just to show that you can
    }),
    helper.accessor("cargo", capitalizeFirstHeader),
    helper.accessor("money", capitalizeFirstHeader),
  ]
  return (
    <HoverInfo
      extraTableProps={{
        tableStyleProps: { size: "narrow" },
        sizeClassName: "is-size-7",
      }}
      position={position}
      data={[data]}
      columns={columns}
    />
  )
}

type NodeHoverInfoData = { good: Good; price: number }
export function NodeHoverInfo({
  position,
  data,
}: {
  position: Point
  data: MNode
}) {
  const helper = createColumnHelper<NodeHoverInfoData>()
  const columnData: NodeHoverInfoData[] = Array.from(
    data.markets.entries()
  ).map(([good, info]) => ({ good, price: info.price }))
  const columns = [helper.accessor("good", {}), helper.accessor("price", {})]
  return (
    <HoverInfo
      title={<p>{capitalize(data.id)}</p>}
      extraTableProps={{
        tableStyleProps: { size: "narrow" },
        sizeClassName: "is-size-7",
        noHeader: true,
      }}
      position={position}
      data={columnData}
      columns={columns}
    />
  )
}
