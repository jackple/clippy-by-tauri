import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification"

let permissionGranted = false

/**
 * 弹出错误信息
 */
function toastError(err: Error | PromiseRejectionEvent) {
  if (!permissionGranted) return
  const errMsg =
    (err as Error)?.message ||
    (err as PromiseRejectionEvent)?.reason?.message ||
    (err as PromiseRejectionEvent)?.reason
  if (errMsg && typeof errMsg === "string") {
    sendNotification({ title: "发送错误", body: errMsg })
  }
}

async function main() {
  window.addEventListener("unhandledrejection", toastError)

  permissionGranted = await isPermissionGranted()
  if (!permissionGranted) {
    const permission = await requestPermission()
    permissionGranted = permission === "granted"
  }
}

main()
