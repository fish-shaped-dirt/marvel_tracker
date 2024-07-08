use rusqlite::{named_params, Connection};
use std::fs;
use tauri::AppHandle;

const CURRENT_DB_VERSION: u32 = 1;

pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("MyApp.sqlite");
    println!("{}", sqlite_path.display());

    let mut db = Connection::open(sqlite_path)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| Ok(row.get(0)?))?;
    drop(user_pragma);

    upgrade_database_if_needed(&mut db, existing_user_version)?;

    Ok(db)
}

pub fn upgrade_database_if_needed(
    db: &mut Connection,
    existing_version: u32,
) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        db.pragma_update(None, "journal_mode", "WAL")?;

        let tx = db.transaction()?;

        tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

        tx.execute_batch(
            "
        CREATE TABLE api (
            id INTEGER PRIMARY KEY,
            client_key TEXT NOT NULL,
            secret_key TEXT NOT NULL
        );
        
        CREATE TABLE serie (
            id INTEGER PRIMARY KEY,
            title VARCHAR(100) NOT NULL,
            desc VARCHAR(1000),
            start_year INTEGER NOT NULL,
            end_year INTEGER,
            image VARCHAR(255) NOT NULL,
            nb_issues INTEGER NOT NULL,
            read_issues INTEGER NOT NULL,
            updated DATETIME,
            status INTEGER NOT NULL,
            favorite BOOLEAN
        );
        
        CREATE TABLE issue (
            id INTEGER PRIMARY KEY,
            title VARCHAR(100) NOT NULL,
            image VARCHAR(255) NOT NULL,
            date DATE NOT NULL,
            read DATETIME,
            status INT NOT NULL,
            serie_id INT NOT NULL,
            FOREIGN KEY(serie_id) REFERENCES serie(id)
        );
        
        CREATE INDEX idx_issue_serie_id ON issue(serie_id);",
        )?;

        tx.commit()?;
    }

    Ok(())
}

pub fn push_api(
    client_key: &str,
    secret_key: &str,
    db: &Connection,
) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare(
        "INSERT OR REPLACE INTO api (id, client_key, secret_key) VALUES (0, @key1, @key2)",
    )?;
    statement.execute(named_params! { "@key1": client_key, "@key2": secret_key })?;

    Ok(())
}

pub fn get_keys(db: &Connection) -> Result<(String, String), rusqlite::Error> {
    let mut statement = db.prepare("SELECT client_key, secret_key FROM api LIMIT 1")?;
    let mut keys = statement.query([])?;
    if let Some(row) = keys.next()? {
        let client_key: String = row.get(0)?;
        let secret_key: String = row.get(1)?;
        Ok((client_key, secret_key))
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

pub fn add_item(title: &str, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare("INSERT INTO items (title) VALUES (@title)")?;
    statement.execute(named_params! { "@title": title })?;

    Ok(())
}

pub fn get_all(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT * FROM items")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        let title: String = row.get("title")?;

        items.push(title);
    }

    Ok(items)
}
