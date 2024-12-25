import { useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api/core"

import { initDatabase, getItems, addItem, type Item } from "./utils/db"
import winAdjust from "./utils/winAdjust"
import styles from "./App.module.scss"
import reactLogo from "./assets/react.svg"

function App() {
  const [greetMsg, setGreetMsg] = useState("")
  const [name, setName] = useState("")
  const [items, setItems] = useState<Item[]>([])

  useEffect(() => {
    initDatabase().then(() => {
      loadItems()
    })

    winAdjust()
  }, [])

  async function loadItems() {
    const data = await getItems()
    setItems(data)
  }

  async function handleAddItem() {
    await addItem("测试标题", "测试内容")
    await loadItems()
  }

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }))
  }

  return (
    <main className={styles.container}>
      <h1>Welcome to Tauri + React</h1>

      <div className={styles.dbTest}>
        <h2>数据库测试</h2>
        <button onClick={handleAddItem}>添加测试数据</button>

        <div className={styles.itemList}>
          {items.map((item) => (
            <div key={item.id} className={styles.item}>
              <h3>{item.title}</h3>
              <p>{item.content}</p>
              <small>{new Date(item.created_at).toLocaleString()}</small>
            </div>
          ))}
        </div>
      </div>

      <div className={styles.row}>
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className={styles.logoVite} alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className={styles.logoTauri} alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className={styles.logoReact} alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className={styles.row}
        onSubmit={(e) => {
          e.preventDefault()
          greet()
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  )
}

export default App
