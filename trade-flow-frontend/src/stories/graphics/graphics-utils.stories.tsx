import React, { useEffect, useState } from "react"
import "bulma/css/bulma.min.css"
import { ComponentMeta } from "@storybook/react"
import { CanvasWithOverlay } from "../canvas"
import { Circle } from "react-konva"
import { PosProps, View } from "./graphics-utils"
import { Html } from "react-konva-utils"
import * as ReactDOM from "react-dom"
import { dbg, UseStateType } from "../../utils"

export default {
  title: "Graphics Utils",
  component: View,
} as ComponentMeta<typeof View>

const Template = (args: {
  main: (args: { posState: UseStateType<Point> }) => JSX.Element
}) => {
  const [position, setPosition] = React.useState({ x: 300, y: 300 })

  return (
    <CanvasWithOverlay
      children={args.main({ posState: [position, setPosition] })}
      OverlayDom={
        <View height={20} position={position}>
          HI
        </View>
      }
      border={true}
    />
  )
}

export const TestCircle = Template.bind({})
TestCircle.args = {
  main: ({ posState: [{ x, y }, setPos] }) => (
    <Circle
      fill={"red"}
      radius={10}
      draggable
      onDragMove={e =>
        setPos({ x: e.target.position().x, y: e.target.position().y })
      }
      x={x}
      y={y}
    ></Circle>
  ),
}

export const PortalExperiment = () => {
  const [node1, setNode1] = useState({ x: 300, y: 200, id: "paris" })
  const [node2, setNode2] = useState({ x: 200, y: 400, id: "toulouse" })
  const portalRef = React.useRef(null)
  useEffect(() => setNode2(node => ({ ...node, y: node.y + 2 })), [])

  return (
    <CanvasWithOverlay
      children={<CircleWithTitle portalRef={portalRef} position={node1} />}
      // OverlayDom={<div>Bye</div>}
      overlayRef={portalRef}
      // stageRef={portalRef}
      border={false}
    />
  )
}

const CircleWithTitle = ({
  position,
  portalRef,
}: PosProps & { portalRef: any }) => {
  // dbg(portalRef.current, "portalRef")
  const [pos, setPos] = useState(position)
  return (
    <>
      <Circle
        draggable
        onDragMove={e =>
          setPos({ x: e.target.position().x, y: e.target.position().y })
        }
        fill="red"
        radius={30}
        {...pos}
      />
      <Html>
        {portalRef.current
          ? ReactDOM.createPortal(
              <View position={pos}>
                <p>hi</p>
              </View>,
              portalRef.current
            )
          : null}
      </Html>
    </>
  )
}
