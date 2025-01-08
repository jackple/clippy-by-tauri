import { Record, RecordType } from "../../utils/db"
import styles from "./styles.module.scss"

interface Props {
  record: Record
}

export function RecordItem({ record }: Props) {
  switch (record.record_type) {
    case RecordType.File:
      return (
        <div className={styles.fileItem}>
          <i className={styles.fileIcon} />
          <div className={styles.fileInfo}>
            <p className={styles.filePath} title={record.value}>
              {record.value}
            </p>
            {record.size && (
              <small>大小: {(record.size / 1024 / 1024).toFixed(2)}MB</small>
            )}
          </div>
        </div>
      )

    case RecordType.Image:
      return (
        <div className={styles.imageItem}>
          {record.thumbnail && (
            <img
              src={`data:image/png;base64,${record.thumbnail}`}
              alt="预览图"
            />
          )}
          {record.img_size && <small>尺寸: {record.img_size}</small>}
        </div>
      )

    case RecordType.Text:
      return (
        <div className={styles.textItem}>
          <p className={styles.textContent}>{record.value}</p>
        </div>
      )

    default:
      return null
  }
}
