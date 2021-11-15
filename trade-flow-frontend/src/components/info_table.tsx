import { isNumber } from "lodash";
import React, {
  FC,
  PropsWithChildren,
  useEffect,
  useRef,
  useState,
} from "react";
import { Table } from "react-bulma-components";
import {
  AgentInfoTable,
  AtPosition,
  CenteredAbove,
  MarketInfoTable,
} from "./info_box";

export enum InfoTableMode {
  cities = "cities",
  agents = "agents",
  singleGood = "singleGood",
}

interface InfoTableProps {
  model: Model;
  oldModel: Model;
  mode: InfoTableMode;
}

export const InfoTable = ({ model, oldModel, mode }: InfoTableProps) => {
  if (mode == InfoTableMode.cities) {
    return (
      <div>
        {Array.from(model.nodes, ([id, node]) => (
          <div key={id} style={{ marginBottom: 5 }}>
            <h4>{id}</h4>
            <MarketInfoTable
              node={node}
              oldMarkets={oldModel.nodes.get(id).markets}
            />
          </div>
        ))}
      </div>
    );
  } else if (mode == InfoTableMode.agents) {
    return (
      <div>
        {Array.from(model.agents, ([id, agent]) => (
          <div key={id} style={{ marginBottom: 5 }}>
            <h4>{id}</h4>
            <AgentInfoTable
              agent={agent}
              oldAgent={oldModel.agents.get(id)}
            />
          </div>
        ))}
      </div>
    );
  }
};

