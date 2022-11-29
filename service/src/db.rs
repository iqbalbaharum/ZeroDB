use marine_rs_sdk::marine;
use marine_sqlite_connector::{Connection, Error, Result, Value};

pub fn get_none_error() -> Error {
    Error {
        code: None,
        message: Some("Value doesn't exist".to_string()),
    }
}

/**
 * Every database will have its own default files separated by database names
 */
pub fn get_connection(db_name: &str) -> Connection {
    let path = format!("tmp/'{}'_db.sqlite", db_name);
    Connection::open(&path).unwrap()
}

pub fn create_dht_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "
		create table if not exists dht (
            uuid INTEGER not null primary key AUTOINCREMENT,
            key TEXT not null unique primary key,
            cid TEXT not null,
            owner_pk TEXT not null
        ) without rowid;
		",
    )?;

    Ok(())
}

pub fn delete_dht_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "
		drop table if exists dht;
		",
    )?;

    Ok(())
}

pub fn add_record(conn: &Connection, key: String, owner_pk: String, cid: String) -> Result<()> {
    conn.execute(format!(
        "
        insert into dht (key, cid, owner_pk)
        values ('{}', '{}', '{}');
        ",
        key, cid, owner_pk
    ))?;

    Ok(())
}

pub fn get_record(conn: &Connection, key: String) -> Result<Record> {
    let mut cursor = conn
        .prepare(format!("select * from dht where key = '{}';", key))?
        .cursor();

    let row = cursor.next()?.ok_or(get_none_error());
    let found_record = Record::from_row(row.unwrap());
    Ok(found_record?)
}

#[marine]
#[derive(Default)]
pub struct Record {
    pub key: String,
    pub cid: String,
    pub public_key: String,
    pub err_msg: String,
    pub success: bool,
}

impl Record {
    pub fn from_row(row: &[Value]) -> Result<Record> {
        let row_record = Record {
            key: row[0].as_string().ok_or(get_none_error())?.to_string(),
            cid: row[1].as_string().ok_or(get_none_error())?.to_string(),
            public_key: row[2].as_string().ok_or(get_none_error())?.to_string(),
            err_msg: "".to_string(),
            success: true,
            ..Default::default()
        };

        Ok(row_record)
    }

    pub fn from_res(res: Result<Record>) -> Record {
        match res {
            Ok(v) => v,
            Err(e) => {
                let mut res_data: Record = Default::default();
                res_data.err_msg = e.to_string();
                res_data.success = false;
                res_data
            }
        }
    }
}
