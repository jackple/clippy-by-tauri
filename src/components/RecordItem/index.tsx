import { useCallback, useRef } from "react"
import { invoke } from "@tauri-apps/api/core"
import classNames from "classnames"

import textIcon from "../../assets/text.png"
import picIcon from "../../assets/pic.png"
import fileIcon from "../../assets/file.png"
import colorIcon from "../../assets/color.png"
import { type Record } from "../../utils/db"
import styles from "./styles.module.scss"

interface Props {
  record: Record
  setRecords: React.Dispatch<React.SetStateAction<Record[]>>
}

export function RecordItem({ record, setRecords }: Props) {
  const getColorValue = (hex: string) => {
    // 如果以#开头，移除#
    let colorValue = hex.startsWith("#") ? hex.substring(1) : hex
    // 如果是3位颜色值，转换为6位
    if (colorValue.length === 3) {
      colorValue = colorValue
        .split("")
        .map((char) => char + char)
        .join("")
    }
    return `#${colorValue}`
  }

  const isColorValue = (value: string) => {
    // 如果以#开头，移除#
    const colorValue = value.startsWith("#") ? value.substring(1) : value
    // 判断是否是3位或6位16进制颜色值
    return /^[0-9A-Fa-f]{3}$|^[0-9A-Fa-f]{6}$/.test(colorValue)
  }

  const getInvertColor = (hex: string) => {
    // 确保有#号
    const colorValue = getColorValue(hex)
    // 移除#号
    const color = colorValue.substring(1)
    // 转换为RGB
    const r = parseInt(color.substring(0, 2), 16)
    const g = parseInt(color.substring(2, 4), 16)
    const b = parseInt(color.substring(4, 6), 16)
    // 计算反色
    const rInvert = (255 - r).toString(16).padStart(2, "0")
    const gInvert = (255 - g).toString(16).padStart(2, "0")
    const bInvert = (255 - b).toString(16).padStart(2, "0")
    // 返回反色的十六进制值
    return `#${rInvert}${gInvert}${bInvert}`
  }

  const getTitle = () => {
    if (record.record_type === "text" && isColorValue(record.value)) {
      return "颜色"
    }
    switch (record.record_type) {
      case "text":
        return "文本"
      case "image":
        return "图像"
      case "file":
        return "文件"
    }
  }

  const getIcon = () => {
    if (record.record_type === "text" && isColorValue(record.value)) {
      return colorIcon
    }
    switch (record.record_type) {
      case "text":
        return textIcon
      case "image":
        return picIcon
      case "file":
        return fileIcon
    }
  }

  const getTime = () => {
    const now = new Date()
    const recordTime = new Date(record.updated_at)
    recordTime.setHours(recordTime.getHours() + 8)

    const diff = now.getTime() - recordTime.getTime()

    if (diff < 60 * 1000) {
      return "刚刚"
    }
    if (diff < 60 * 60 * 1000) {
      return `${Math.floor(diff / (60 * 1000))}分钟前`
    }
    if (diff < 24 * 60 * 60 * 1000) {
      return `${Math.floor(diff / (60 * 60 * 1000))}小时前`
    }
    return `${Math.floor(diff / (24 * 60 * 60 * 1000))}天前`
  }

  const isColor = record.record_type === "text" && isColorValue(record.value)

  const favoriteRef = useRef<HTMLDivElement>(null)

  const toggleFavorite = useCallback(
    async (e: React.MouseEvent) => {
      e.stopPropagation() // 阻止事件冒泡，避免触发选中效果
      await invoke("toggle_favorite", { id: record.id })
      // 立即更新当前列表中的收藏状态
      setRecords((prevRecords) =>
        prevRecords.map((r) =>
          r.id === record.id ? { ...r, favorite: !r.favorite } : r
        )
      )
    },
    [record.id, setRecords]
  )

  return (
    <div
      className={styles.container}
      style={isColor ? { background: getColorValue(record.value) } : undefined}
    >
      <div className={styles.header}>
        <div className={styles.title}>
          <span>{getTitle()}</span>
          <img src={getIcon()} alt="icon" width={24} height={24} />
        </div>
        <div className={styles.timeWrapper}>
          <div className={styles.time}>{getTime()}</div>
          <div
            ref={favoriteRef}
            className={classNames(styles.favoriteIcon, {
              [styles.active]: record.favorite,
            })}
            onClick={toggleFavorite}
            onDoubleClick={(e) => e.stopPropagation()}
          >
            {record.favorite ? "★" : "☆"}
            <div className={styles.tooltip}>
              {record.favorite ? "取消收藏" : "收藏"}
            </div>
          </div>
        </div>
      </div>
      <div className={styles.content}>
        {!!record.is_deleted && (
          <div className={styles.error}>文件已不存在</div>
        )}
        {record.record_type === "image" ? (
          <img
            src={`data:image/png;base64,${record.thumbnail}`}
            alt="thumbnail"
          />
        ) : (
          <div
            className={styles.text}
            style={
              isColor
                ? {
                    color: getInvertColor(record.value),
                    fontSize: 24,
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    height: "100%",
                  }
                : undefined
            }
          >
            {record.display_text || record.value}
          </div>
        )}
      </div>
      {!isColor && (
        <div className={styles.meta}>
          {record.record_type === "text" && (
            <span>{record.value.length}个字符</span>
          )}
          {record.record_type === "file" && record.size && (
            <span>{formatSize(record.size)}</span>
          )}
          {record.record_type === "image" && record.img_size && (
            <span>{record.img_size}</span>
          )}
        </div>
      )}
    </div>
  )
}

function formatSize(bytes: number): string {
  const units = ["B", "KB", "MB", "GB"]
  let size = bytes
  let unitIndex = 0

  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024
    unitIndex++
  }

  return `${size.toFixed(1)} ${units[unitIndex]}`
}
