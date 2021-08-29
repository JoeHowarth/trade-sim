import {isNumber} from "lodash";
import React, {
  FC,
  PropsWithChildren,
  useEffect,
  useRef,
  useState,
} from "react";
import {Table} from "react-bulma-components";
import {AtPosition, CenteredAbove, MarketInfoTable} from "./info_box";

interface InfoTableProps {
  model: Model
  oldModel: Model
}

const InfoTable = ({model, oldModel}: InfoTableProps) => {
  return (
    <div onClick={() => console.warn("I've been clicked 2")}>
      {
        Array.from(model.nodes, ([id, node]) => (
          <div key={id} style={{marginBottom: 5}}>
            <h4>{id}</h4>
            <MarketInfoTable  node={node} oldMarkets={oldModel.nodes.get(id).markets}/>
          </div>
        ))
      }
    </div>
  )
}

export
{
  InfoTable
}