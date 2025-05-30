import styles from "./styles.module.scss"

export function EmptyState() {
  return (
    <div className={styles.emptyState}>
      <i className={styles.emptyIcon} />
      <p>暂无记录</p>
    </div>
  )
}
