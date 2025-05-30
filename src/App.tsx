import { useCallback, useEffect, useState, useRef } from "react"
import { debounce } from "lodash-es"
import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event"

import { getRecords, type Record, RecordType } from "./utils/db"
import { Search } from "./components/Search"
import { RecordList, type RecordListRef } from "./components/RecordList"
import Updater from "./components/Updater"
import styles from "./App.module.scss"

const LIMIT = 30

function App() {
  const [records, setRecords] = useState<Record[]>([])
  const [selectedId, setSelectedId] = useState<number | null>(null)
  const [keyword, setKeyword] = useState("")
  const [selectedType, setSelectedType] = useState<
    RecordType | "all" | "favorite"
  >("all")
  const [hasMore, setHasMore] = useState(true)
  const listRef = useRef<RecordListRef>(null)

  const handleSearch = useCallback(
    (value: string, type: RecordType | "all" | "favorite") => {
      setKeyword(value)
      setSelectedType(type)
      setSelectedId(null) // 重置选中状态
    },
    []
  )

  const debouncedLoadRef = useRef(
    debounce(async (kw: string, type: RecordType | "all" | "favorite") => {
      const data = await getRecords({
        limit: LIMIT,
        keyword: kw,
        record_type: type === "all" || type === "favorite" ? undefined : type,
        favorite: type === "favorite",
      })
      setRecords(data)
      setHasMore(data.length === LIMIT)
      if (data.length > 0) {
        setSelectedId(data[0].id)
      }
    }, 200)
  )

  useEffect(() => {
    debouncedLoadRef.current(keyword, selectedType)
  }, [keyword, selectedType])

  const loadRecords = useCallback(async () => {
    // 检查是否有新的记录, 如果没有, 则不更新
    const data = await getRecords({
      limit: LIMIT,
      keyword,
      record_type:
        selectedType === "all" || selectedType === "favorite"
          ? undefined
          : selectedType,
      favorite: selectedType === "favorite",
    })
    if (records.length) {
      if (data[0]?.id === records[0].id) return
    }

    setRecords(data)
    setHasMore(data.length === LIMIT)
    if (data.length) {
      setSelectedId(data[0].id)
    }
  }, [keyword, records, selectedType])

  const loadMore = useCallback(async () => {
    if (!hasMore) return

    const lastRecord = records[records.length - 1]
    if (!lastRecord) return

    const data = await getRecords({
      last_updated_at: lastRecord.updated_at,
      limit: LIMIT,
      keyword,
      record_type:
        selectedType === "all" || selectedType === "favorite"
          ? undefined
          : selectedType,
      favorite: selectedType === "favorite",
    })

    setRecords((prev) => [...prev, ...data])
    setHasMore(data.length === LIMIT)
  }, [hasMore, keyword, records, selectedType])

  useEffect(() => {
    function handleFocus() {
      loadRecords()
      listRef.current?.focus()
    }

    window.addEventListener("focus", handleFocus, false)
    return () => {
      window.removeEventListener("focus", handleFocus, false)
    }
  }, [loadRecords])

  const handleSelect = useCallback((record: Record) => {
    setSelectedId(record.id)
  }, [])

  useEffect(() => {
    function handleKeyDown(e: KeyboardEvent) {
      if (e.key === "Escape") {
        invoke("toggle_panel")
      }
    }

    const unlisten = listen("history-cleared", () => location.reload())

    window.addEventListener("keydown", handleKeyDown)
    return () => {
      window.removeEventListener("keydown", handleKeyDown)
      unlisten.then((f) => f())
    }
  }, [])

  return (
    <main className={styles.container}>
      <Updater />
      <Search onSearch={handleSearch} />
      <RecordList
        ref={listRef}
        records={records}
        setRecords={setRecords}
        selectedId={selectedId}
        onSelect={handleSelect}
        onLoadMore={loadMore}
      />
    </main>
  )
}

export default App
