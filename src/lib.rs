use rusqlite::{params, Connection, Result};

/// Creates a `students` table in the database if it doesn't already exist.
/// The table includes the following columns:
/// - `id` (INTEGER, PRIMARY KEY): Unique identifier for each student.
/// - `name` (TEXT, NOT NULL): Student's name.
/// - `age` (INTEGER): Student's age.
/// - `major` (TEXT): Student's major.
///
/// Prints a success message if the table is created successfully.
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

/// Inserts a new student into the `students` table with the provided `id`, `name`, `age`, and `major`.
///
/// # Arguments
/// - `id`: Unique identifier for the student.
/// - `name`: Name of the student.
/// - `age`: Age of the student.
/// - `major`: Major of the student.
///
/// Prints a success message if the student is added successfully.
pub fn create_student(conn: &Connection, id: i32, name: &str, age: i32, major: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO students (id, name, age, major) VALUES (?1, ?2, ?3, ?4)",
        params![id, name, age, major],
    )?;
    println!("Student added successfully.");
    Ok(())
}

/// Retrieves and prints the details of a student with the given `id` from the `students` table.
/// The output is formatted as a table with columns for `id`, `name`, `age`, and `major`.
///
/// # Arguments
/// - `id`: Unique identifier of the student to be read.
///
/// Prints the student details in a formatted table.
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

/// Retrieves and prints the details of all students from the `students` table.
/// The output is formatted as a table with columns for `id`, `name`, `age`, and `major`.
///
/// Prints all student records in a formatted table.
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

/// Updates a specific column of a student record identified by `id` in the `students` table.
///
/// # Arguments
/// - `id`: Unique identifier of the student to update.
/// - `column`: The column name to update (e.g., `name`, `age`, `major`).
/// - `value`: The new value to set for the specified column.
///
/// Prints a success message if the update is successful.
pub fn update_student(conn: &Connection, id: i32, column: &str, value: &str) -> Result<()> {
    let sql = format!("UPDATE students SET {} = ?1 WHERE id = ?2", column);
    conn.execute(&sql, params![value, id])?;
    println!("Student information updated successfully.");
    Ok(())
}

/// Deletes a student record identified by `id` from the `students` table.
///
/// # Arguments
/// - `id`: Unique identifier of the student to delete.
///
/// Prints a success message if the student is deleted successfully.
pub fn delete_student(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM students WHERE id = ?1", params![id])?;
    println!("Student deleted successfully.");
    Ok(())
}

/// Deletes the `students` table if it exists.
///
/// Prints a success message if the table is deleted successfully.
pub fn delete_table(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE IF EXISTS students", [])?;
    println!("Table deleted successfully.");
    Ok(())
}

/// Executes a custom SQL query on the database. If the query is a `SELECT` statement,
/// it retrieves and prints the results in a formatted table. For other types of SQL commands,
/// it simply executes them.
///
/// # Arguments
/// - `sql`: The SQL query to execute.
///
/// If the query is a `SELECT`, it prints the result as a table; otherwise, it prints a success message.
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
