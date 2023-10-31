use rusqlite::{params, Connection, Result, Error};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "query")]
    Query,
    #[structopt(name = "insert")]
    Insert {
        id: String,
        age: i64,
    },
    #[structopt(name = "update")]
    Update {
        id: String,
        age: i64,
    },
    #[structopt(name = "delete")]
    Delete {
        id: String,
    },
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS details (
            id INTEGER PRIMARY KEY,
            age INTEGER
        )",
        [],
    )?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Cli = Cli::from_args();

    let conn = Connection::open("details.db")?;
    create_table(&conn)?;

    match args.cmd {
        Command::Query => {
            let mut stmt = conn.prepare("SELECT id, age FROM details LIMIT 10")?;
            let rows = stmt.query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
            })?;

            for row in rows {
                let (id, age) = row?;
                println!("id: {}, age: {}", id, age);
            }
        }
        Command::Insert { id, age } => {
            let rows_affected = conn.execute("INSERT INTO details (id, age) VALUES (?1, ?2)", params![id, age])?;

            if rows_affected == 1 {
                println!("Insert operation completed successfully");
            } else {
                println!("Insert operation did not affect any rows");
            }
        }
        Command::Update { id, age } => {
            let rows_affected = conn.execute("UPDATE details SET age = ?1 WHERE id = ?2", params![age, id])?;

            if rows_affected == 1 {
                println!("Update operation completed successfully");
            } else {
                println!("Update operation did not affect any rows");
            }
        }
        Command::Delete { id } => {
            let rows_affected = conn.execute("DELETE FROM details WHERE id = ?1", params![id])?;

            if rows_affected == 1 {
                println!("Delete operation completed successfully");
            } else {
                println!("Delete operation did not affect any rows");
            }
        }
    }

    Ok(())
}
