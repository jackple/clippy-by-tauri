use lazy_static::lazy_static;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::Manager;

lazy_static! {
    static ref DB: Mutex<Option<Database>> = Mutex::new(None);
}

#[derive(Debug, Serialize)]
pub struct Record {
    id: i64,
    record_type: String,
    value: String,
    thumbnail: Option<String>,
    size: Option<i64>,
    img_size: Option<String>,
    created_at: String,
    updated_at: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    fn get() -> Result<std::sync::MutexGuard<'static, Option<Database>>> {
        let db = DB.lock().unwrap();
        if db.is_some() {
            Ok(db)
        } else {
            Err(rusqlite::Error::InvalidParameterName(
                "Database not initialized".to_string(),
            ))
        }
    }
}

pub fn init(app: &tauri::App) {
    let app_dir = app.app_handle().path().app_data_dir().unwrap();
    std::fs::create_dir_all(&app_dir).unwrap();
    let db_path = app_dir.join("app.db");

    let conn = Connection::open(db_path).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS record (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            record_type TEXT CHECK(record_type IN ('text', 'image', 'file')) NOT NULL,
            value TEXT NOT NULL,
            thumbnail TEXT,
            size INTEGER,
            img_size TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .unwrap();

    let mut db = DB.lock().unwrap();
    *db = Some(Database { conn });
}

#[derive(Debug, Deserialize)]
pub struct RecordInput {
    record_type: String,
    value: String,
    thumbnail: Option<String>,
    size: Option<i64>,
    img_size: Option<String>,
}

#[tauri::command]
pub async fn add_record(record: RecordInput) -> Result<i64, String> {
    println!("add_record: {:?}", record);
    let db = Database::get().map_err(|e| e.to_string())?;
    let db = db.as_ref().unwrap();

    db.conn
        .execute(
            "INSERT INTO record (record_type, value, thumbnail, size, img_size, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, CURRENT_TIMESTAMP)",
            (
                record.record_type,
                record.value,
                record.thumbnail,
                record.size,
                record.img_size,
            ),
        )
        .map_err(|e| e.to_string())?;

    Ok(db.conn.last_insert_rowid())
}

#[tauri::command]
pub async fn get_records() -> Result<Vec<Record>, String> {
    let db = Database::get().map_err(|e| e.to_string())?;
    let db = db.as_ref().unwrap();

    let mut stmt = db
        .conn
        .prepare(
            "SELECT id, record_type, value, thumbnail, size, img_size, created_at, updated_at 
             FROM record ORDER BY updated_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let records = stmt
        .query_map([], |row| {
            Ok(Record {
                id: row.get(0)?,
                record_type: row.get(1)?,
                value: row.get(2)?,
                thumbnail: row.get(3)?,
                size: row.get(4)?,
                img_size: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    records
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_record(id: i64) -> Result<(), String> {
    let db = Database::get().map_err(|e| e.to_string())?;
    let db = db.as_ref().unwrap();

    db.conn
        .execute("DELETE FROM record WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
