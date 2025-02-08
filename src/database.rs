// TODO: filter?
// TESTING:
// PERF: conn and disconn in operations automatically?

/******************************************************************************
* This module is the interfaces with database
* The function "disconnect_to_db" should be called once after all interactions
* The initialization of the database should call the function "create_table"
* The Operations include:
*   INSERT: insert_task
*   FETCH: fetch_task & fetch_task_by_index 
*   UPDATE: update_task 
*   DELETE: delete_task
******************************************************************************/
#[allow(dead_code)]
pub mod database {
    use rusqlite::{Connection, Result};
    use crate::task::task::Task;
    use serde_rusqlite::*;

    const DATABASE_PATH: &str = "/home/fs002905/.taskoto/taskoto.db";

    pub fn connect_to_db() -> Result<Connection> {
        let conn = Connection::open(DATABASE_PATH)?;
        Ok(conn)
    }

    pub fn disconnect_to_db(conn: Connection) {
        let _ = conn.close();
    }

    pub fn create_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks(
                    id INTEGER PRIMARY KEY, 
                    name TEXT NOT NULL, 
                    status TEXT NOT NULL,
                    due TEXT,
                    scheduled TEXT,
                    start_time TEXT,
                    end_time TEXT,
                    project TEXT,
                    _is_started INTEGER NOT NULL)",
            [],
        ).unwrap();
        Ok(())
    }

    pub fn insert_task(conn: &Connection, task: &Task) -> Result<()> {
        conn.execute(
            "INSERT INTO tasks (
                name, status, due, scheduled, start_time, end_time, 
                project, _is_started) VALUES (
                :name, :status, :due, :scheduled, :start_time, :end_time, :project, :_is_started)",
            to_params_named_with_fields(task, 
                &["name", "status", "due", "scheduled", 
                            "start_time", "end_time", "project", "_is_started"])
                .unwrap().to_slice().as_slice()
        ).unwrap();
        Ok(())
    }
    
    pub fn fetch_task(conn: &Connection) -> Result<Vec<Task>> {
        let mut stmt = conn.prepare("SELECT * FROM tasks").unwrap();
        let task = from_rows::<Task>(stmt.query([]).unwrap());
        let mut tasks: Vec<Task> = Vec::new();
        for i in task {
            let mut tmp = i.unwrap();
            tmp.verify();
            tasks.push(tmp);
        }
        Ok(tasks)
    }    

    pub fn fetch_task_by_index(conn: &Connection, id: i32) -> Result<Task> {
        let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id=?1")?;
        let mut task = from_rows::<Task>(stmt.query([id]).unwrap());
        let mut tmp = task.next().unwrap().unwrap();
        tmp.verify();
        Ok(tmp)
    }

    pub fn update_task(conn: &Connection, task: &Task) -> Result<()> {
         conn.execute(
            "UPDATE tasks SET name=:name, status=:status, due=:due,
                    scheduled=:scheduled, start_time=:start_time,
                    end_time=:end_time, project=:project, _is_started=:_is_started
                    WHERE id=:id",
            to_params_named_with_fields(task, 
                &["name", "status", "due", "scheduled", "start_time",
                            "end_time", "project", "_is_started", "id"]
            ).unwrap().to_slice().as_slice()
        ).unwrap();
        Ok(())
    }

    pub fn delete_task(conn: &Connection, id: i32) -> Result<()> {
        let mut stmt= conn.prepare("DELETE FROM tasks WHERE id=?1").unwrap();
        stmt.execute([id]).unwrap();
        Ok(())
    }

    pub fn delete_all(conn: &Connection) -> Result<()> {
        // conn.execute("DELETE * FROM tasks", []).unwrap();
        let mut stmt= conn.prepare("DELETE FROM tasks").unwrap();
        stmt.execute([]).unwrap();
        Ok(())
    }
}
