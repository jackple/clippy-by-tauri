import { useCallback, useEffect, useState, useRef } from "react"
import { debounce } from "lodash-es"

import { getRecords, type Record } from "./utils/db"
import { SearchInput } from "./components/SearchInput"
import { RecordList, type RecordListRef } from "./components/RecordList"
import styles from "./App.module.scss"

const LIMIT = 30

function App() {
  const [records, setRecords] = useState<Record[]>([])
  const [selectedId, setSelectedId] = useState<number | null>(null)
  const [keyword, setKeyword] = useState("")
  const [hasMore, setHasMore] = useState(true)
  const listRef = useRef<RecordListRef>(null)

  const debouncedLoadRef = useRef(
    debounce(async (kw: string) => {
      const data = await getRecords({
        limit: LIMIT,
        keyword: kw,
      })
      setRecords(data)
      setHasMore(data.length === LIMIT)
      if (data.length > 0) {
        setSelectedId(data[0].id)
      }
    }, 200)
  )

  useEffect(() => {
    debouncedLoadRef.current(keyword)
  }, [keyword])

  const handleSearch = useCallback((value: string) => {
    setKeyword(value)
    setSelectedId(null) // 重置选中状态
  }, [])

  const loadRecords = useCallback(async () => {
    // 检查是否有新的记录, 如果有, 则重新请求并把第一个选中
    if (records.length) {
      const data0 = await getRecords({
        limit: 1,
        keyword,
      })
      if (data0[0]?.id === records[0].id) return
    }

    const data = await getRecords({
      limit: LIMIT,
      keyword,
    })
    setRecords(data)
    setHasMore(data.length === LIMIT)
    if (data.length) {
      setSelectedId(data[0].id)
    }
  }, [keyword, records])

  const loadMore = useCallback(async () => {
    if (!hasMore) return

    const lastRecord = records[records.length - 1]
    if (!lastRecord) return

    const data = await getRecords({
      last_updated_at: lastRecord.updated_at,
      limit: LIMIT,
      keyword,
    })

    setRecords((prev) => [...prev, ...data])
    setHasMore(data.length === LIMIT)
  }, [hasMore, keyword, records])

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

  return (
    <main className={styles.container}>
      <SearchInput onSearch={handleSearch} />
      <RecordList
        ref={listRef}
        records={records}
        selectedId={selectedId}
        onSelect={handleSelect}
        onLoadMore={loadMore}
      />
    </main>
  )
}

export default App
