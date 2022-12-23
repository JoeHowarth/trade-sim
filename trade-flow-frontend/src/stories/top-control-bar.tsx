import React, { ReactChild } from "react"
import { Box, Heading, Level, Button, Icon } from "react-bulma-components"
// import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import {
  faCoffee,
  faPause,
  faBackward,
  faForward,
  faPlay,
} from "@fortawesome/free-solid-svg-icons"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"

export interface TickControls {
  tick: number
  isPaused: boolean
  togglePlay()
  faster()
  slower()
}

export interface ViewControls<View extends string> {
  setActiveView(name: View)
  views: View[]
}

function TopControlBar<View extends string>(
  props: {
    title: string
    onClickExit?: React.MouseEventHandler<HTMLButtonElement>
  } & TickControls &
    ViewControls<View>
): JSX.Element {
  const { title, tick, onClickExit } = props
  return (
    <Box
      style={{
        width: "100%",
        pointerEvents: "auto",
        borderBottom: "2px solid #DDD",
      }}
    >
      <Level className={"is-mobile"}>
        <Level.Side align="left">
          <Level.Item>
            <Heading size={3} weight={"semibold"}>
              {title}
            </Heading>
          </Level.Item>
          {props.views.map(v => (
            <Level.Item key={v}>
              <Button onClick={() => props.setActiveView(v)}>{v}</Button>
            </Level.Item>
          ))}
        </Level.Side>
        <Level.Side align="right">
          <Level.Item style={{ width: 80 }} justifyContent="flex-start">
            {`Tick: ${tick}`}{" "}
          </Level.Item>
          <Level.Item>
            <Button onClick={() => props.slower()}>
              <Icon>
                <FontAwesomeIcon icon={faBackward} />
              </Icon>
            </Button>
          </Level.Item>
          <Level.Item>
            <Button onClick={() => props.togglePlay()}>
              <Icon>
                <FontAwesomeIcon icon={props.isPaused ? faPause : faPlay} />
              </Icon>
            </Button>
          </Level.Item>
          <Level.Item>
            <Button onClick={() => props.faster()}>
              <Icon>
                <FontAwesomeIcon icon={faForward} />
              </Icon>
            </Button>
          </Level.Item>
        </Level.Side>
      </Level>
    </Box>
  )
}

export default TopControlBar
