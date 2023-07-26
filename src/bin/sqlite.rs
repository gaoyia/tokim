use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Msg {
    id: i32,
    from: i32,
    to: i32,
    data: Option<Vec<u8>>,
}

fn insert_msg(conn: &Connection, msg: &Msg) -> Result<()> {
    conn.execute(
        "INSERT INTO msg (\"from\", \"to\", data) VALUES (?1, ?2, ?3)",
        (&msg.from, &msg.to, &msg.data),
    )?;
    Ok(())
}

fn query_msgs(conn: &Connection, from: Option<i32>, to: Option<i32>) -> Result<Vec<Msg>> {
    let query = "SELECT id, \"from\", \"to\", data FROM msg";
    let where_clause = match (from, to) {
        (Some(from), Some(to)) => format!(" WHERE \"from\" = {} AND \"to\" = {}", from, to),
        (Some(from), None) => format!(" WHERE \"from\" = {}", from),
        (None, Some(to)) => format!(" WHERE \"to\" = {}", to),
        (None, None) => String::new(),
    };
    let final_query = format!("{}{}", query, where_clause);

    let mut stmt = conn.prepare(&final_query)?;
    let msg_iter = stmt.query_map([], |row| {
        Ok(Msg {
            id: row.get(0)?,
            from: row.get(1)?,
            to: row.get(2)?,
            data: row.get(3)?,
        })
    })?;

    let mut msgs = Vec::new();
    for msg in msg_iter {
        msgs.push(msg?);
    }
    Ok(msgs)
}

// 更新数据
fn update_msg(conn: &Connection, msg: &Msg) -> Result<()> {
    conn.execute(
        "UPDATE msg SET \"from\" = ?1, \"to\" = ?2, data = ?3 WHERE id = ?4",
        (&msg.from, &msg.to, &msg.data, &msg.id),
    )?;
    Ok(())
}

// 删除数据

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE msg (
            id    INTEGER PRIMARY KEY,
            \"from\"  INTEGER NOT NULL,
            \"to\"    INTEGER NOT NULL,
            data  BLOB
        )",
        (),
    )?;

    // 添加消息
    let msg1 = Msg {
        id: 1,
        from: 0,
        to: 1,
        data: None,
    };
    insert_msg(&conn, &msg1)?;

    let msg2 = Msg {
        id: 2,
        from: 1,
        to: 0,
        data: None,
    };
    insert_msg(&conn, &msg2)?;

    // 查询所有消息
    let msgs = query_msgs(&conn, None, None)?;
    for msg in msgs {
        println!("Found msg {:?}", msg);
    }

    // 更新消息
    let updated_msg = Msg {
        id: 1,
        from: 0,
        to: 2,
        data: Some(vec![1, 2, 3]),
    };
    update_msg(&conn, &updated_msg)?;

    // 查询更新后的消息
    let updated_msgs = query_msgs(&conn, None, None)?;
    for msg in updated_msgs {
        println!("Found updated msg {:?}", msg);
    }

    // 删除消息
    // delete_msg(&conn, 2)?;

    // 查询删除后的消息
    let remaining_msgs = query_msgs(&conn, None, None)?;
    for msg in remaining_msgs {
        println!("Found remaining msg {:?}", msg);
    }

    Ok(())
}