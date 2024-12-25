import Database from "@tauri-apps/plugin-sql"

let db: Database | null = null

export async function initDatabase() {
  if (!db) {
    db = await Database.load("sqlite:app.db")
    await db.execute(`
      CREATE TABLE IF NOT EXISTS items (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        content TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )
    `)
  }
  return db
}

export interface Item {
  id: number
  title: string
  content: string
  created_at: string
}

export async function addItem(title: string, content: string) {
  const db = await initDatabase()
  return db.execute(
    "INSERT INTO items (title, content) VALUES ($1, $2) RETURNING id",
    [title, content]
  )
}

export async function getItems(): Promise<Item[]> {
  const db = await initDatabase()
  const result = await db.select<Item[]>(
    "SELECT * FROM items ORDER BY created_at DESC"
  )
  return result
}
