import { useCallback, useEffect, useState, useRef } from "react"

import { getRecords, type Record } from "./utils/db"
import { SearchInput } from "./components/SearchInput"
import { RecordList, type RecordListRef } from "./components/RecordList"
import styles from "./App.module.scss"

function App() {
  const [records, setRecords] = useState<Record[]>([])
  const [selectedId, setSelectedId] = useState<number | null>(null)
  const [keyword, setKeyword] = useState("")
  const listRef = useRef<RecordListRef>(null)

  const loadRecords = useCallback(async () => {
    const data = await getRecords({ page: 1, page_size: 30, keyword })
    setRecords(data)
    // 默认选中第一条记录
    if (data.length > 0 && !selectedId) {
      setSelectedId(data[0].id)
    }
  }, [selectedId, keyword])

  const handleSearch = useCallback((value: string) => {
    setKeyword(value)
    setSelectedId(null) // 重置选中状态
  }, [])

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
      />
    </main>
  )
}

export default App
