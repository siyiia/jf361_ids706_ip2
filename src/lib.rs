use rusqlite::{params, Connection, Result};

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS students (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER,
            major TEXT
        )",
        [],
    )?;
    println!("Table created successfully.");
    Ok(())
}

pub fn create_student(conn: &Connection, id: i32, name: &str, age: i32, major: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO students (id, name, age, major) VALUES (?1, ?2, ?3, ?4)",
        params![id, name, age, major],
    )?;
    println!("Student added successfully.");
    Ok(())
}

pub fn read_student(conn: &Connection, id: i32) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, age, major FROM students WHERE id = ?1")?;

    println!(
        "| {:<3} | {:<12} | {:<3} | {:<6} |",
        "id", "name", "age", "major"
    );

    let student_iter = stmt.query_map([id], |row| {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let age: i32 = row.get(2)?;
        let major: String = row.get(3)?;
        Ok((id, name, age, major))
    })?;

    for student in student_iter {
        let (id, name, age, major) = student?;
        println!("| {:<3} | {:<12} | {:<3} | {:<6} |", id, name, age, major);
    }
    Ok(())
}

pub fn read_all_students(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, age, major FROM students")?;

    println!(
        "| {:<3} | {:<12} | {:<3} | {:<6} |",
        "id", "name", "age", "major"
    );

    let student_iter = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let age: i32 = row.get(2)?;
        let major: String = row.get(3)?;
        Ok((id, name, age, major))
    })?;

    for student in student_iter {
        let (id, name, age, major) = student?;
        println!("| {:<3} | {:<12} | {:<3} | {:<6} |", id, name, age, major);
    }
    Ok(())
}

pub fn update_student(conn: &Connection, id: i32, column: &str, value: &str) -> Result<()> {
    let sql = format!("UPDATE students SET {} = ?1 WHERE id = ?2", column);
    conn.execute(&sql, params![value, id])?;
    println!("Student information updated successfully.");
    Ok(())
}

pub fn delete_student(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM students WHERE id = ?1", params![id])?;
    println!("Student deleted successfully.");
    Ok(())
}

pub fn delete_table(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE IF EXISTS students", [])?;
    println!("Table deleted successfully.");
    Ok(())
}

pub fn execute_query(conn: &Connection, sql: &str) -> Result<()> {
    if sql.trim().to_uppercase().starts_with("SELECT") {
        let mut stmt = conn.prepare(sql)?;

        println!(
            "| {:<3} | {:<12} | {:<3} | {:<6} |",
            "id", "name", "age", "major"
        );

        let student_iter = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let name: String = row.get(1)?;
            let age: i32 = row.get(2)?;
            let major: String = row.get(3)?;
            Ok((id, name, age, major))
        })?;

        for student in student_iter {
            let (id, name, age, major) = student?;
            println!("| {:<3} | {:<12} | {:<3} | {:<6} |", id, name, age, major);
        }
    } else {
        conn.execute_batch(sql)?;
        println!("Query executed successfully.");
    }
    Ok(())
}
