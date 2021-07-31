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
    <div>
      {
        Array.from(model.nodes, (node, idx) => (
          <div key={idx} style={{marginBottom: 5}}>
            <h4>{node.id}</h4>
            <MarketInfoTable  node={node} oldMarkets={oldModel.nodes[idx].markets}/>
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