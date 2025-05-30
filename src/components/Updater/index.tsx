import { useEffect, useState, useCallback, useRef } from "react"
import { check } from "@tauri-apps/plugin-updater"
import { relaunch } from "@tauri-apps/plugin-process"
import styles from "./styles.module.scss"

function Updater() {
  const [needRestart, setNeedRestart] = useState(false)
  const timerRef = useRef<number>()
  const isPending = useRef(false)

  const checkForUpdate = useCallback(async () => {
    if (!navigator.onLine) return
    if (isPending.current) return

    isPending.current = true
    try {
      const update = await check({
        target: "macos-universal",
      })

      console.log("check update result", update)

      if (update) {
        await update.downloadAndInstall()
        setNeedRestart(true)
        // 清除定时器
        if (timerRef.current) {
          clearInterval(timerRef.current)
          timerRef.current = undefined
        }
      }
    } finally {
      isPending.current = false
    }
  }, [])

  useEffect(() => {
    checkForUpdate()
    timerRef.current = setInterval(checkForUpdate, 5 * 60 * 1000)
    return () => {
      if (timerRef.current) {
        clearInterval(timerRef.current)
      }
    }
  }, [checkForUpdate])

  if (!needRestart) return null

  return (
    <div className={styles.updater}>
      <span>最新版本已安装完成!</span>
      <button onClick={relaunch}>立即重启</button>
    </div>
  )
}

export default Updater
