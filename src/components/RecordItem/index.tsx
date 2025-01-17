import textIcon from "../../assets/text.png"
import picIcon from "../../assets/pic.png"
import fileIcon from "../../assets/file.png"
import { type Record } from "../../utils/db"
import styles from "./styles.module.scss"

interface Props {
  record: Record
}

export function RecordItem({ record }: Props) {
  const getTitle = () => {
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
    const recordTime = new Date(record.created_at)
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

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.title}>
          <span>{getTitle()}</span>
          <img src={getIcon()} alt="icon" width={24} height={24} />
        </div>
        <div className={styles.time}>{getTime()}</div>
      </div>
      <div className={styles.content}>
        {record.record_type === "image" ? (
          <img
            src={`data:image/png;base64,${record.thumbnail}`}
            alt="thumbnail"
          />
        ) : (
          <div className={styles.text}>{record.value}</div>
        )}
      </div>
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
