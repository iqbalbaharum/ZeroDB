use marine_rs_sdk::marine;
use marine_sqlite_connector::{Connection, Error, Result, Value};

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
