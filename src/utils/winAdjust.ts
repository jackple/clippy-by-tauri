import {
  getCurrentWindow,
  LogicalPosition,
  Monitor,
  LogicalSize,
} from "@tauri-apps/api/window"
import { invoke } from "@tauri-apps/api/core"

const WIN_HEIGHT = 322

async function winAdjust() {
  const monitor = await invoke<Monitor>("get_active_monitor")
  // console.log("monitor", monitor)
  const logicalWidth = monitor.size.width / monitor.scaleFactor
  const logicalHeight = monitor.size.height / monitor.scaleFactor

  const win = getCurrentWindow()
  win.setSize(new LogicalSize(logicalWidth, WIN_HEIGHT))
  win.setPosition(
    new LogicalPosition(
      monitor.position.x,
      logicalHeight - WIN_HEIGHT + monitor.position.y
    )
  )
}

export default winAdjust
