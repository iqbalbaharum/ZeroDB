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
            key TEXT not null,
            cid TEXT not null,
            owner_pk TEXT not null
        );
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
        "insert into dht (key, cid, owner_pk) values ('{}', '{}', '{}');",
        key, cid, owner_pk
    ))?;

    println!(
        "insert into dht (key, cid, owner_pk) values ('{}', '{}', '{}');",
        key, cid, owner_pk
    );

    Ok(())
}

pub fn get_records(conn: &Connection) -> Result<Vec<Record>> {
    let mut cursor = conn.prepare("select * from dht;")?.cursor();

    let mut records = Vec::new();
    while let Some(row) = cursor.next()? {
        records.push(Record::from_row(row)?);
    }

    Ok(records)
}

pub fn update_record(conn: &Connection, owner_pk: String, cid: String) -> Result<()> {
    conn.execute(format!(
        "
        update dht 
        set cid = '{}' 
        where owner_pk = '{}';
        ",
        cid, owner_pk
    ))?;

    Ok(())
}

pub fn get_record(conn: &Connection, key: String) -> Result<Record> {
    read_execute(conn, format!("select * from dht where key = '{}';", key))
}

pub fn get_record_by_pk(conn: &Connection, pk: String) -> Result<Option<Record>> {
    let mut cursor = conn
        .prepare(format!("select * from dht where owner_pk = '{}';", pk))?
        .cursor();

    let row = cursor.next()?;
    if row != None {
        let found_record = Record::from_row(row.unwrap());
        Ok(Some(found_record.unwrap()))
    } else {
        Ok(None)
    }
}

fn read_execute(conn: &Connection, statement: String) -> Result<Record> {
    let mut cursor = conn.prepare(statement)?.cursor();
    let row = cursor.next()?.ok_or(get_none_error());
    let found_record = Record::from_row(row.unwrap_or_default());
    Ok(found_record?)
}

#[marine]
#[derive(Default, PartialEq)]
pub struct Record {
    pub uuid: i64,
    pub key: String,
    pub cid: String,
    pub public_key: String,
    pub err_msg: String,
    pub success: bool,
}

impl Record {
    pub fn from_row(row: &[Value]) -> Result<Record> {
        let row_record = Record {
            uuid: row[0].as_integer().ok_or(get_none_error())?,
            key: row[1].as_string().ok_or(get_none_error())?.to_string(),
            cid: row[2].as_string().ok_or(get_none_error())?.to_string(),
            public_key: row[3].as_string().ok_or(get_none_error())?.to_string(),
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
