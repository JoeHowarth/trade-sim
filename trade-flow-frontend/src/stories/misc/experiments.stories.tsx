import { ComponentMeta } from "@storybook/react"
import React, { useState, useEffect } from "react"
import { Box } from "react-bulma-components"

export default {
  title: "Experiments",
  // component: PropsAsInitialStateExperiment,
} as ComponentMeta<typeof Box>

export const PropsAsInitialStateExperiment = () => {
  const [num, setNum] = useState(3)
  console.log(num, "parent")
  return (
    <Box onClick={() => setNum(num => num + 10)}>
      <Dummy num={num} />
    </Box>
  )
}

const Dummy = (props: { num: number }) => {
  const [num, setNum] = useState(props.num)
  useEffect(() => setNum(props.num), [props.num])
  console.log(num, "child")
  return (
    <div
      onClick={e => {
        e.stopPropagation()
        setNum(x => x + 1)
      }}
    >
      {num}
    </div>
  )
}
