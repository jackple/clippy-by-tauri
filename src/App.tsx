import { useCallback, useEffect, useState } from "react"

import { getRecords, type Record } from "./utils/db"
import styles from "./App.module.scss"

function App() {
  const [records, setRecords] = useState<Record[]>([])

  const loadRecords = useCallback(async () => {
    const data = await getRecords({ page: 1, page_size: 30 })
    setRecords(data)
  }, [])

  useEffect(() => {
    function handleFocus() {
      console.log("handleFocus")
      loadRecords()
    }

    window.addEventListener("focus", handleFocus, false)

    return () => {
      window.removeEventListener("focus", handleFocus, false)
    }
  }, [loadRecords])

  return (
    <main className={styles.container}>
      <div className={styles.dbTest}>
        <h2>数据库测试 {records.length}</h2>

        <div className={styles.itemList}>
          {records.map((record) => (
            <div key={record.id} className={styles.item}>
              <h3>{record.record_type}</h3>
              <p>{record.value}</p>
              {record.thumbnail && (
                <img src={record.thumbnail} alt="thumbnail" />
              )}
              {record.size && <small>Size: {record.size}</small>}
              {record.img_size && <small>Image size: {record.img_size}</small>}
              <div>
                <small>
                  Created: {new Date(record.created_at).toLocaleString()}
                </small>
                <small>
                  Updated: {new Date(record.updated_at).toLocaleString()}
                </small>
              </div>
            </div>
          ))}
        </div>
      </div>
    </main>
  )
}

export default App
