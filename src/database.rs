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
    use crate::project::project::Project;
    use serde_rusqlite::*;
    use crate::*;

    const DATABASE_NAME: &str = "taskoto.db";

    pub fn connect_to_db() -> Result<Connection> {
        let conn = Connection::open(&(get_database_dir() + DATABASE_NAME))?;
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
                    project_id, INTEGER,
                    project TEXT,
                    _is_started INTEGER NOT NULL,
                    _urgent REAL NOT NULL)",
            [],
        ).unwrap();
        Ok(())
    }

    pub fn create_project_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects(
                    id INTEGER PRIMARY KEY, 
                    name TEXT NOT NULL, 
                    deadline TEXT,
                    description TEXT,
                    is_done INTEGER NOT NULL)",
            [],
        ).unwrap();
        Ok(())
    }

    pub fn create_trigger(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TRIGGER update_project_name
                    AFTER UPDATE OF name ON projects
                    FOR EACH ROW
                    BEGIN
                        UPDATE tasks
                        SET project = NEW.name
                        WHERE project_id = OLD.id;
                    END",
            [],
        ).unwrap();
        Ok(())
    }


    pub fn insert_task(conn: &Connection, task: &Task) -> Result<()> {
        conn.execute(
            "INSERT INTO tasks (
                name, status, due, scheduled, start_time, end_time, 
                project_id, project, _is_started, _urgent) VALUES (
                :name, :status, :due, :scheduled, :start_time, :end_time, 
                :project_id, :project, :_is_started, :_urgent)",
            to_params_named_with_fields(task, 
                &["name", "status", "due", "scheduled", "start_time", 
                    "end_time","project_id", "project", "_is_started", "_urgent"])
                .unwrap().to_slice().as_slice()
        ).unwrap();
        Ok(())
    }

    pub fn insert_project(conn: &Connection, project: &Project) -> Result<()> {
        conn.execute(
            "INSERT INTO projects(
                name, deadline, description, is_done) VALUES ( 
                :name, :deadline, :description, :is_done)",
            to_params_named_with_fields(project, 
                &["name", "deadline", "description", "is_done"])
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

    pub fn fetch_project(conn: &Connection) -> Result<Vec<Project>> {
        let mut stmt = conn.prepare("SELECT * FROM projects").unwrap();
        let pro = from_rows::<Project>(stmt.query([]).unwrap());
        let mut projects: Vec<Project> = Vec::new();
        for i in pro{
            projects.push(i.unwrap());
        }
        Ok(projects)
    }    

    pub fn fetch_task_by_index(conn: &Connection, id: i32) -> Result<Task, ()> {
        let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id=?1").unwrap();
        let mut rows = from_rows::<Task>(stmt.query([id]).unwrap());
        match rows.next() {
            Some(task) => {
                let mut tmp = task.unwrap();
                tmp.verify();
                Ok(tmp)
            },
            None => Err(()),
        }
    }
    
    pub fn fetch_project_by_index(conn: &Connection, id: i32) -> Result<Project, ()> {
        let mut stmt = conn.prepare("SELECT * FROM projects WHERE id=?1").unwrap();
        let mut rows = from_rows::<Project>(stmt.query([id]).unwrap());
        match rows.next() {
            Some(pro) => Ok(pro.unwrap()), 
            None => Err(()),
        }
    }

    pub fn update_task(conn: &Connection, task: &Task) -> Result<()> {
         conn.execute(
            "UPDATE tasks SET name=:name, status=:status, due=:due,
                    scheduled=:scheduled, start_time=:start_time,
                    end_time=:end_time, project_id=:project_id, project=:project, 
                    _is_started=:_is_started, _urgent=:_urgent
                    WHERE id=:id",
            to_params_named_with_fields(task, 
                &["name", "status", "due", "scheduled", "start_time",
                            "end_time", "project_id", "project",
                            "_is_started", "_urgent", "id"]
            ).unwrap().to_slice().as_slice()
        ).unwrap();
        Ok(())
    }
    
    pub fn update_project(conn: &Connection, project: &Project) -> Result<()> {
         conn.execute(
            "UPDATE projects SET name=:name, deadline=:deadline,
                    description=:description, is_done=:is_done
                    WHERE id=:id",
            to_params_named_with_fields(project, 
                &["name", "deadline", "description", "is_done", "id"]
            ).unwrap().to_slice().as_slice()
        ).unwrap();
        Ok(())
    }

    pub fn delete_project(conn: &Connection, id: i32) -> Result<(), ()> {
        let mut stmt= conn.prepare("DELETE FROM projects WHERE id=?1").unwrap();
        match stmt.execute([id]) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn delete_task(conn: &Connection, id: i32) -> Result<(), ()> {
        let mut stmt= conn.prepare("DELETE FROM tasks WHERE id=?1").unwrap();
        match stmt.execute([id]) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn delete_all(conn: &Connection) -> Result<()> {
        let mut stmt= conn.prepare("DELETE FROM tasks").unwrap();
        stmt.execute([]).unwrap();
        Ok(())
    }

    pub fn delete_all_projects(conn: &Connection) -> Result<()> {
        let mut stmt= conn.prepare("DELETE FROM projects").unwrap();
        stmt.execute([]).unwrap();
        Ok(())
    }

}
