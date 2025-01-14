import { useCallback, useEffect, useState, useRef } from "react"
import { debounce } from "lodash-es"

import { getRecords, type Record } from "./utils/db"
import { SearchInput } from "./components/SearchInput"
import { RecordList, type RecordListRef } from "./components/RecordList"
import styles from "./App.module.scss"

function App() {
  const [records, setRecords] = useState<Record[]>([])
  const [selectedId, setSelectedId] = useState<number | null>(null)
  const [keyword, setKeyword] = useState("")
  const [page, setPage] = useState(1)
  const [hasMore, setHasMore] = useState(true)
  const listRef = useRef<RecordListRef>(null)
  const PAGE_SIZE = 30

  const debouncedLoadRef = useRef(
    debounce(async (kw: string) => {
      const data = await getRecords({
        page: 1,
        page_size: PAGE_SIZE,
        keyword: kw,
      })
      setRecords(data)
      setPage(1)
      setHasMore(data.length === PAGE_SIZE)
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
    const data = await getRecords({ page: 1, page_size: PAGE_SIZE, keyword })
    setRecords(data)
    setPage(1)
    setHasMore(data.length === PAGE_SIZE)
    if (data.length > 0 && !selectedId) {
      setSelectedId(data[0].id)
    }
  }, [keyword])

  const loadMore = useCallback(async () => {
    if (!hasMore) return

    const nextPage = page + 1
    const data = await getRecords({
      page: nextPage,
      page_size: PAGE_SIZE,
      keyword,
    })

    setRecords((prev) => [...prev, ...data])
    setPage(nextPage)
    setHasMore(data.length === PAGE_SIZE)
  }, [page, hasMore, keyword])

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
