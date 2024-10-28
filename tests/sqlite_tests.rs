use sqlite::{
    create_student, create_table, delete_student, delete_table, execute_query, read_all_students,
    read_student, update_student,
};

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_in_memory_db() -> Connection {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        create_table(&conn).expect("Failed to create table");
        conn
    }

    #[test]
    fn test_create_table() {
        let conn = setup_in_memory_db();
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='students';")
            .unwrap();
        let table_exists = stmt.exists([]).expect("Failed to execute query");

        assert!(table_exists, "Table should be created successfully");
    }

    #[test]
    fn test_create_student() {
        let conn = setup_in_memory_db();
        let result = create_student(&conn, 1, "Alice", 20, "Computer Science");
        assert!(result.is_ok(), "Student should be added successfully");

        let mut stmt = conn
            .prepare("SELECT id, name, age, major FROM students WHERE id = 1")
            .unwrap();
        let student = stmt.query_row([], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i32>(2)?,
                row.get::<_, String>(3)?,
            ))
        });

        match student {
            Ok((id, name, age, major)) => {
                assert_eq!(id, 1);
                assert_eq!(name, "Alice");
                assert_eq!(age, 20);
                assert_eq!(major, "Computer Science");
            }
            Err(_) => panic!("Student not found"),
        }
    }

    #[test]
    fn test_read_student() {
        let conn = setup_in_memory_db();
        create_student(&conn, 1, "Alice", 20, "Computer Science").unwrap();

        let result = read_student(&conn, 1);
        assert!(result.is_ok(), "Student should be read successfully");
    }

    #[test]
    fn test_read_all_students() {
        let conn = setup_in_memory_db();
        create_student(&conn, 1, "Alice", 20, "Computer Science").unwrap();
        create_student(&conn, 2, "Bob", 22, "Mathematics").unwrap();

        let result = read_all_students(&conn);
        assert!(result.is_ok(), "All students should be read successfully");
    }

    #[test]
    fn test_update_student() {
        let conn = setup_in_memory_db();
        create_student(&conn, 1, "Alice", 20, "Computer Science").unwrap();

        let result = update_student(&conn, 1, "major", "Mathematics");
        assert!(result.is_ok(), "Student should be updated successfully");

        let mut stmt = conn
            .prepare("SELECT major FROM students WHERE id = 1")
            .unwrap();
        let major: String = stmt.query_row([], |row| row.get(0)).unwrap();
        assert_eq!(
            major, "Mathematics",
            "Student's major should be updated to Mathematics"
        );
    }

    #[test]
    fn test_delete_student() {
        let conn = setup_in_memory_db();
        create_student(&conn, 1, "Alice", 20, "Computer Science").unwrap();

        let result = delete_student(&conn, 1);
        assert!(result.is_ok(), "Student should be deleted successfully");

        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM students WHERE id = 1")
            .unwrap();
        let count: i32 = stmt.query_row([], |row| row.get(0)).unwrap();
        assert_eq!(count, 0, "Student with id 1 should not exist");
    }

    #[test]
    fn test_delete_table() {
        let conn = setup_in_memory_db();

        delete_table(&conn).expect("Failed to delete table");

        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='students';")
            .unwrap();
        let table_exists = stmt.exists([]).expect("Failed to execute query");

        assert!(!table_exists, "Table should no longer exist");
    }

    #[test]
    fn test_execute_query() {
        let conn = setup_in_memory_db();
        create_student(&conn, 1, "Alice", 20, "Computer Science").unwrap();

        let result = execute_query(&conn, "SELECT * FROM students WHERE id = 1");
        assert!(result.is_ok(), "Execute query should run successfully");
    }
}
