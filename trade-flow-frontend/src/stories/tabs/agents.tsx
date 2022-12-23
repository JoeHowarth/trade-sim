import { createColumnHelper } from "@tanstack/react-table"
import React, { Dispatch, useState } from "react"
import { Container, Icon } from "react-bulma-components"
import { dbg, SetState } from "../../utils"
import { TabEnum } from "../main-view"
import BasicTable, { capitalizeFirstHeader, col } from "../misc/basic-table"
import { mockMAgents } from "../misc/mocks"
import OverlayWindow from "./overlay-window"

type States = "agents" | { id: AgentId }

export const AgentsTab = ({
  setActiveView,
  agents,
  getModels,
}: {
  setActiveView: SetState<TabEnum>
  agents: Map<AgentId, MAgent>
  getModels(): Model[]
}) => {
  const [state, setState] = useState<States>("agents")
  let content = null
  if (state == "agents") {
    content = (
      <BasicTable
        columns={[
          {
            accessorKey: "id",
            cell: info => (
              <div onClick={() => setState({ id: info.getValue() as string })}>
                {info.getValue()}
              </div>
            ),
          },
          c("location"),
          c("cargo"),
          c("money"),
        ]}
        tableStyleProps={{ size: "fullwidth" }}
        defaultData={dbg(Array.from(agents.values()))}
      />
    )
  } else if (state?.id) {
    const models = getModels()
    const agentHistory = models.map(m => {
      return m.agents.get(state.id)
    })
    content = (
      <>
        <BasicTable
          columns={[c("location"), c("cargo"), c("money")]}
          defaultData={agentHistory}
        />
      </>
    )
  }
  return (
    <OverlayWindow
      title={
        <Icon color="text">
          <span>Agents</span>
          {state?.id?}
        </Icon>
      }
      onClickExit={() => setActiveView(null)}
    >
      {content}
    </OverlayWindow>
  )
}

function c<T>(name: keyof T): {
  id: string
  accessorKey: string | number | symbol
} & typeof capitalizeFirstHeader {
  return {
    id: String(name),
    accessorKey: name,
    ...capitalizeFirstHeader,
  }
}
