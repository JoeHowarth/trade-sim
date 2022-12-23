import React, { Dispatch } from "react"
import { SetState } from "../../utils"
import { TabEnum } from "../main-view"
import BasicTable from "../misc/basic-table"
import OverlayWindow from "./overlay-window"

export const CitiesTab = ({
  setActiveView,
  nodes,
}: {
  setActiveView: SetState<TabEnum>
  nodes: MNode[]
}) => {
  const citiesView = nodes.map(n => {
    const markets = Object.fromEntries(
      Array.from(n.markets.entries()).map(([good, info]) => {
        return [good, info.price]
      })
    )
    return { city: n.id, ...markets }
  })
  return (
    <OverlayWindow title="Cities" onClickExit={() => setActiveView(null)}>
      <BasicTable
        tableStyleProps={{ size: "fullwidth" }}
        defaultData={[...citiesView]}
      />
    </OverlayWindow>
  )
}
