use clap::{Arg, Command};
use rusqlite::Connection;
use sqlite::{
    create_student, create_table, delete_student, delete_table, execute_query, read_all_students,
    read_student, update_student,
};

fn main() -> rusqlite::Result<()> {
    let matches = Command::new("Student Database CLI")
        .version("1.0")
        .author("Jingjing Feng")
        .about("Manages a student database")
        .subcommand(Command::new("create_table").about("Creates the students table"))
        .subcommand(
            Command::new("create")
                .about("Adds a new student")
                .arg(Arg::new("id").required(true))
                .arg(Arg::new("name").required(true))
                .arg(Arg::new("age").required(true))
                .arg(Arg::new("major").required(true)),
        )
        .subcommand(
            Command::new("read")
                .about("Reads student data")
                .arg(Arg::new("id").required(false)),
        )
        .subcommand(
            Command::new("update")
                .about("Updates a student's information")
                .arg(Arg::new("id").required(true))
                .arg(Arg::new("column").required(true))
                .arg(Arg::new("value").required(true)),
        )
        .subcommand(
            Command::new("delete_row")
                .about("Deletes a student by ID")
                .arg(Arg::new("id").required(true)),
        )
        .subcommand(Command::new("delete_table").about("Deletes the entire students table"))
        .subcommand(
            Command::new("query")
                .about("Executes a custom SQL query")
                .arg(Arg::new("sql").required(true)),
        )
        .get_matches();

    let conn = Connection::open("students.db")?;

    match matches.subcommand() {
        Some(("create_table", _)) => create_table(&conn)?,
        Some(("create", sub_m)) => {
            let id: i32 = sub_m
                .get_one::<String>("id")
                .expect("ID is required")
                .parse()
                .expect("ID should be an integer");
            let name = sub_m
                .get_one::<String>("name")
                .expect("Name is required")
                .as_str();
            let age: i32 = sub_m
                .get_one::<String>("age")
                .expect("Age is required")
                .parse()
                .expect("Invalid age");
            let major = sub_m
                .get_one::<String>("major")
                .expect("Major is required")
                .as_str();
            create_student(&conn, id, name, age, major)?;
        }
        Some(("read", sub_m)) => {
            if let Some(id_str) = sub_m.get_one::<String>("id") {
                let id: i32 = id_str.parse().expect("ID should be an integer");
                read_student(&conn, id)?;
            } else {
                read_all_students(&conn)?;
            }
        }
        Some(("update", sub_m)) => {
            let id: i32 = sub_m
                .get_one::<String>("id")
                .expect("ID is required")
                .parse()
                .expect("ID should be an integer");
            let column = sub_m
                .get_one::<String>("column")
                .expect("Column is required")
                .as_str();
            let value = sub_m
                .get_one::<String>("value")
                .expect("Value is required")
                .as_str();
            update_student(&conn, id, column, value)?;
        }
        Some(("delete_row", sub_m)) => {
            let id: i32 = sub_m
                .get_one::<String>("id")
                .expect("ID is required")
                .parse()
                .expect("ID should be an integer");
            delete_student(&conn, id)?;
        }
        Some(("delete_table", _)) => delete_table(&conn)?,
        Some(("query", sub_m)) => {
            let sql = sub_m
                .get_one::<String>("sql")
                .expect("SQL query is required")
                .as_str();
            execute_query(&conn, sql)?;
        }
        _ => println!("Unknown command"),
    }

    Ok(())
}
