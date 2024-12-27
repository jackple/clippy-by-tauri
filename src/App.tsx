import { useEffect, useState } from "react"
import { readImage } from "@tauri-apps/plugin-clipboard-manager"

import { getRecords, addRecord, type Record, RecordType } from "./utils/db"
import styles from "./App.module.scss"

function App() {
  const [records, setRecords] = useState<Record[]>([])

  useEffect(() => {
    loadRecords()

    readImage().then((image) => {
      console.log(image)
    })
  }, [])

  async function loadRecords() {
    const data = await getRecords()
    setRecords(data)
  }

  async function handleAddTestRecord() {
    await addRecord({
      type: RecordType.Text,
      value: "测试内容",
    })
    await loadRecords()
  }

  return (
    <main className={styles.container}>
      <h1>Welcome to Tauri + React</h1>

      <div className={styles.dbTest}>
        <h2>数据库测试</h2>
        <button onClick={handleAddTestRecord}>添加测试数据</button>

        <div className={styles.itemList}>
          {records.map((record) => (
            <div key={record.id} className={styles.item}>
              <h3>{record.type}</h3>
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
