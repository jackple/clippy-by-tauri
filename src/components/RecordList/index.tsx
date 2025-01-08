import {
  useCallback,
  useRef,
  useImperativeHandle,
  forwardRef,
  useEffect,
} from "react"
import classNames from "classnames"
import { type Record } from "../../utils/db"
import { RecordItem } from "../RecordItem"
import styles from "./styles.module.scss"

interface Props {
  records: Record[]
  selectedId: number | null
  onSelect: (record: Record, index: number) => void
}

export interface RecordListRef {
  focus: () => void
}

export const RecordList = forwardRef<RecordListRef, Props>(function RecordList(
  { records, selectedId, onSelect },
  ref
) {
  const listRef = useRef<HTMLDivElement>(null)

  // 滚动选中的项目到合适位置
  const scrollSelectedIntoView = useCallback((index: number) => {
    if (!listRef.current) return

    const container = listRef.current
    const item = container.children[index] as HTMLElement
    // 使用 scrollIntoView 确保元素可见
    item.scrollIntoView({
      behavior: "smooth",
      block: "nearest",
      inline: "center", // 尝试将元素居中显示
    })
  }, [])

  // 处理键盘事件
  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (records.length === 0) return

      const currentIndex = records.findIndex((r) => r.id === selectedId)
      if (currentIndex === -1) return

      switch (e.key) {
        case "ArrowLeft": {
          if (currentIndex > 0) {
            const record = records[currentIndex - 1]
            onSelect(record, currentIndex - 1)
          }
          break
        }
        case "ArrowRight": {
          if (currentIndex < records.length - 1) {
            const record = records[currentIndex + 1]
            onSelect(record, currentIndex + 1)
          }
          break
        }
      }
    },
    [records, selectedId, onSelect]
  )

  // 监听键盘事件
  useEffect(() => {
    window.addEventListener("keydown", handleKeyDown)
    return () => window.removeEventListener("keydown", handleKeyDown)
  }, [handleKeyDown])

  // 当选中项改变时，确保它在可视区域内
  useEffect(() => {
    if (selectedId === null || records.length === 0) return
    const selectedIndex = records.findIndex((r) => r.id === selectedId)
    if (selectedIndex !== -1) {
      scrollSelectedIntoView(selectedIndex)
    }
  }, [selectedId, records, scrollSelectedIntoView])

  useImperativeHandle(ref, () => ({
    focus: () => {
      listRef.current?.focus()
    },
  }))

  const handleItemClick = useCallback(
    (record: Record, index: number) => {
      onSelect(record, index)
    },
    [onSelect]
  )

  return (
    <div ref={listRef} className={styles.recordList} tabIndex={-1}>
      {records.map((record, index) => (
        <div
          key={record.id}
          className={classNames(styles.recordItem, {
            [styles.selected]: record.id === selectedId,
          })}
          onClick={() => handleItemClick(record, index)}
        >
          <RecordItem record={record} />
          <div className={styles.recordMeta}>
            <time>{new Date(record.updated_at).toLocaleString()}</time>
          </div>
        </div>
      ))}
    </div>
  )
})
