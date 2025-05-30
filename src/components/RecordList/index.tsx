import {
  useCallback,
  useRef,
  useImperativeHandle,
  forwardRef,
  useEffect,
} from "react"
import { invoke } from "@tauri-apps/api/core"
import classNames from "classnames"
import { debounce } from "lodash-es"

import { type Record } from "../../utils/db"
import { RecordItem } from "../RecordItem"
import { EmptyState } from "../EmptyState"
import styles from "./styles.module.scss"

interface Props {
  records: Record[]
  setRecords: React.Dispatch<React.SetStateAction<Record[]>>
  selectedId: number | null
  onSelect: (record: Record, index: number) => void
  onLoadMore: () => void
}

export interface RecordListRef {
  focus: () => void
}

export const RecordList = forwardRef<RecordListRef, Props>(function RecordList(
  { records, setRecords, selectedId, onSelect, onLoadMore },
  ref
) {
  const listRef = useRef<HTMLDivElement>(null)
  const clickTimer = useRef<ReturnType<typeof setTimeout> | null>(null)

  const handleClick: (record: Record, index: number) => void = (
    record,
    index
  ) => {
    if (clickTimer.current) {
      // 如果已经有计时器，说明是双击，清除计时器并返回
      clearTimeout(clickTimer.current)
      clickTimer.current = null
      return
    }

    // 设置计时器，延迟执行单击事件
    clickTimer.current = setTimeout(() => {
      onSelect(record, index)
      clickTimer.current = null
    }, 150) // 150ms 的延迟，在响应速度和双击检测之间取得平衡
  }

  const handleDoubleClick = (record: Record) => {
    if (clickTimer.current) {
      clearTimeout(clickTimer.current)
      clickTimer.current = null
    }
    choose(record)
  }

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
            e.preventDefault()
            const record = records[currentIndex - 1]
            onSelect(record, currentIndex - 1)
          }
          break
        }
        case "ArrowRight": {
          if (currentIndex < records.length - 1) {
            e.preventDefault()
            const record = records[currentIndex + 1]
            onSelect(record, currentIndex + 1)
          }
          break
        }
        case "Enter": {
          if (currentIndex !== -1) {
            choose(records[currentIndex])
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

  async function choose(record: Record) {
    await invoke("choose", { record }).catch((e) => {
      if (e === "File not found") {
        setRecords((rs) => {
          const index = rs.findIndex((r) => r.id === record.id)
          if (index === -1) return rs
          rs[index] = { ...rs[index], is_deleted: true }
          return [...rs]
        })
      }
      throw e
    })
    await invoke("toggle_panel")
  }

  // 监听滚动事件
  const handleScroll = useCallback(
    debounce(() => {
      if (!listRef.current) return

      const container = listRef.current
      const clientWidth = container.clientWidth

      // 当滚动到距离右边 50% 的位置时加载更多
      if (
        container.scrollWidth - (container.scrollLeft + clientWidth) <
        clientWidth * 0.5
      ) {
        onLoadMore()
      }
    }, 200),
    [onLoadMore]
  )

  useEffect(() => {
    const container = listRef.current
    if (!container) return

    container.addEventListener("scroll", handleScroll)
    return () => container.removeEventListener("scroll", handleScroll)
  }, [handleScroll])

  return (
    <div ref={listRef} className={styles.recordList} tabIndex={-1}>
      {records.length === 0 ? (
        <EmptyState />
      ) : (
        records.map((record, index) => (
          <div
            key={record.id}
            className={classNames(styles.recordItem, {
              [styles.selected]: selectedId === record.id,
              [styles.deleted]: record.is_deleted,
            })}
            onClick={() => handleClick(record, index)}
            onDoubleClick={() => handleDoubleClick(record)}
          >
            <RecordItem record={record} setRecords={setRecords} />
          </div>
        ))
      )}
    </div>
  )
})
