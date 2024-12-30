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
    // 文件size, 单位bytes
    size: Option<i64>,
    // 图片尺寸 款x高
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
    pub record_type: String,
    pub value: String,
    pub thumbnail: Option<String>,
    pub size: Option<u64>,
    pub img_size: Option<String>,
}

// 通过 record_type 和 value 检查是否存在相同的记录
pub fn check_record_exists(
    conn: &Connection,
    record_type: &str,
    value: &str,
) -> Result<Option<i64>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id FROM record 
         WHERE record_type = ?1 AND value = ?2 
         ORDER BY updated_at DESC LIMIT 1",
    )?;

    Ok(stmt.query_row([record_type, value], |row| row.get(0)).ok())
}

#[tauri::command]
pub async fn add_record(record: RecordInput) -> Result<i64, String> {
    let db = Database::get().map_err(|e| e.to_string())?;
    let db = db.as_ref().unwrap();

    let existing_id = check_record_exists(&db.conn, &record.record_type, &record.value)
        .map_err(|e| e.to_string())?;

    if let Some(id) = existing_id {
        // 如果存在，更新时间戳
        db.conn
            .execute(
                "UPDATE record SET updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
                [id],
            )
            .map_err(|e| e.to_string())?;
        return Ok(id);
    }
    // 如果不存在，插入新记录
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

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub page: u32,
    pub keyword: Option<String>,
}

#[tauri::command]
pub async fn get_records(params: QueryParams) -> Result<Vec<Record>, String> {
    let db = Database::get().map_err(|e| e.to_string())?;
    let db = db.as_ref().unwrap();

    let page_size = 30;
    let offset = (params.page - 1) * page_size;

    // 当type=image时, value是图片base64, 数据太大了, 置为空字符串(渲染时用thumbnail够了)
    let base_query = "SELECT id, record_type, 
             CASE 
                WHEN record_type = 'image' THEN ''
                ELSE value 
             END as value,
             thumbnail, size, img_size, created_at, updated_at 
             FROM record";

    let (query, params) = if let Some(keyword) = params.keyword {
        (
            format!("{} WHERE record_type IN ('text', 'file') AND value LIKE ?1 ORDER BY updated_at DESC LIMIT ?2 OFFSET ?3", base_query),
            vec![
                format!("%{}%", keyword),
                page_size.to_string(),
                offset.to_string(),
            ],
        )
    } else {
        (
            format!("{} ORDER BY updated_at DESC LIMIT ?1 OFFSET ?2", base_query),
            vec![page_size.to_string(), offset.to_string()],
        )
    };

    let mut stmt = db.conn.prepare(query.as_str()).map_err(|e| e.to_string())?;

    let records = stmt
        .query_map(rusqlite::params_from_iter(params), |row| {
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
