import { useCallback, useEffect, useState, useRef } from "react"
import { debounce } from "lodash-es"

import { getRecords, type Record } from "./utils/db"
import { SearchInput } from "./components/SearchInput"
import { RecordList, type RecordListRef } from "./components/RecordList"
import styles from "./App.module.scss"

const DEFAULT_LIMIT = 30

function App() {
  const [records, setRecords] = useState<Record[]>([])
  const [selectedId, setSelectedId] = useState<number | null>(null)
  const [keyword, setKeyword] = useState("")
  const [hasMore, setHasMore] = useState(true)
  const listRef = useRef<RecordListRef>(null)
  const pageLimit = useRef(DEFAULT_LIMIT)

  const debouncedLoadRef = useRef(
    debounce(async (kw: string) => {
      const data = await getRecords({
        limit: pageLimit.current,
        keyword: kw,
      })
      setRecords(data)
      setHasMore(data.length === pageLimit.current)
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
    if (records.length > DEFAULT_LIMIT) {
      pageLimit.current = records.length
    }
    const data = await getRecords({
      limit: pageLimit.current,
      keyword,
    })
    setRecords(data)
    setHasMore(data.length === pageLimit.current)
    if (data.length > 0 && !selectedId) {
      setSelectedId(data[0].id)
    }
    if (pageLimit.current !== DEFAULT_LIMIT) {
      pageLimit.current = DEFAULT_LIMIT
    }
  }, [keyword, records])

  const loadMore = useCallback(async () => {
    if (!hasMore) return

    const lastRecord = records[records.length - 1]
    if (!lastRecord) return

    const data = await getRecords({
      last_updated_at: lastRecord.updated_at,
      limit: pageLimit.current,
      keyword,
    })

    setRecords((prev) => [...prev, ...data])
    setHasMore(data.length === pageLimit.current)
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

  const handleSelect = useCallback(
    (record: Record) => {
      setSelectedId(record.id)
    },
    [records, loadMore]
  )

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
