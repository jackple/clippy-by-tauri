import Database from "@tauri-apps/plugin-sql"

export enum RecordType {
  Text = "text",
  Image = "image",
  File = "file",
}

export interface Record {
  id: number
  type: RecordType
  value: string
  thumbnail?: string
  size?: number
  img_size?: string
  created_at: string
  updated_at: string
}

let db: Database | null = null

export async function initDatabase() {
  if (!db) {
    db = await Database.load("sqlite:app.db")
    await db.execute(`
      CREATE TABLE IF NOT EXISTS record (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        type TEXT CHECK(type IN ('${RecordType.Text}', '${RecordType.Image}', '${RecordType.File}')) NOT NULL,
        value TEXT NOT NULL,
        thumbnail TEXT,
        size INTEGER,
        img_size TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )
    `)
  }
  return db
}

export async function addRecord(
  record: Omit<Record, "id" | "created_at" | "updated_at">
) {
  const db = await initDatabase()
  return db.execute(
    `INSERT INTO record (type, value, thumbnail, size, img_size, updated_at) 
     VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)`,
    [record.type, record.value, record.thumbnail, record.size, record.img_size]
  )
}

export async function getRecords(): Promise<Record[]> {
  const db = await initDatabase()
  return db.select<Record[]>("SELECT * FROM record ORDER BY updated_at DESC")
}

export async function getRecordsByType(type: RecordType): Promise<Record[]> {
  const db = await initDatabase()
  return db.select<Record[]>(
    "SELECT * FROM record WHERE type = $1 ORDER BY updated_at DESC",
    [type]
  )
}

export async function deleteRecord(id: number) {
  const db = await initDatabase()
  return db.execute("DELETE FROM record WHERE id = $1", [id])
}
