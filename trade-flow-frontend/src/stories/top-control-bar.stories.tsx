import "bulma/css/bulma.min.css"
import { ComponentMeta } from "@storybook/react"
import TopControlBar from "./top-control-bar"

export default {
  title: "TopControlBar",
  component: TopControlBar,
} as ComponentMeta<typeof TopControlBar>

export const Main = TopControlBar.bind({})
const args: Parameters<typeof TopControlBar>[0] = {
  togglePlay: () => alert("Pause!"),
  faster: () => alert("Faster!"),
  slower: () => alert("Slower!"),
  tick: 2,
  title: "TradeSim",
  views: ["Agents", "Cities"],
  setActiveView: view => alert(view),
  onClickExit: () => alert("Exit!"),
  isPaused: false,
}
Main.args = args
