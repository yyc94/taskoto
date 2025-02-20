pub mod parser {
    use clap::{Parser, Subcommand};
    use rusqlite::Connection;
    use tabled::{Table, settings::Style};
    use std::io;


    use crate::*;
    use crate::database::database::*;
    use crate::task::task::{self, Task, StateWord, TaskStatus, Filter};
    use crate::project::project::Project;

    const DATE_TYPE_INSTRU: &str = 
    "There are 11 types of date format.\nPlease use the format you specified when \'add\' or \'modify\' a task
        Examples:
        type-1 : 2025-12-31 
        type-2 : 12-31-2025 
        type-3 : 25-12-31 
        type-4 : 12-31-25 
        type-5 : December 31, 2025 
        type-6 : December 31, 25 
        type-7 : Dec 31, 2025 
        type-8 : Dec 31, 25 
        type-9 : 12-31 
        type-10: December 31 
        type-11: Dec 31";

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Option<Command>,
    }
    #[derive(Subcommand)]
    pub enum Command {
        #[command(
            about = "Modify the configuration of the taskoto", 
            long_about = DATE_TYPE_INSTRU
        )]
        Config {

            #[arg(short, long)]
            path: Option<String>,

            #[arg(short, long)]
            date_format: Option<usize>,

            //...
        },
        #[command(about = "Show the configuration.")]
        ShowConfig {

        },
        #[command(about = "Initialize the database at the specify location")]
        Init {},
        #[command(about = "Add a task")]
        Add {
            name: String,

            #[arg(short, long)]
            due: Option<String>,

            #[arg(short, long)]
            scheduled: Option<String>,

            #[arg(short, long)]
            project_id: Option<i32>,

        },
        AddProject {
            name: String,

            #[arg(short, long)]
            deadline: Option<String>,

            #[arg(long)]
            description: Option<String>, 
        },

        #[command(about = "Show the task (id) or all pending tasks")]
        Show {
            id: Option<u8>,
            #[arg(short, long, value_enum)]
            filter: Option<Filter>,
            #[arg(short, long)]
            project: bool,
            #[arg(short, long)]
            all: bool,
        },
        #[command(about = "Modify the task (id)")]
        Modify {
            id: u8,

            #[arg(short, long)]
            name: Option<String>,

            #[arg(short, long)]
            due: Option<String>,

            #[arg(short, long)]
            scheduled: Option<String>,

            #[arg(short, long)]
            project_id: Option<i32>,

        },

        #[command(about = "Modify the project (id)")]
        ModifyProject {
            id: u8,

            #[arg(short, long)]
            name: Option<String>,

            #[arg(short, long)]
            deadline: Option<String>,

            #[arg(long)]
            description: Option<String>,
        },

        #[command(about = "Change the status of the task (id) to CANCELED")]
        Delete {
            id: u8,

            #[arg(short, long)]
            project: bool,
        },

        #[command(about = "Delete the task (id) from the database (CAN NOT BE RECOVERED)")]
        Destroy {
            id: u8,

            #[arg(short, long)]
            project: bool,

        },
        #[command(about = "Delete all tasks or projects from the database (CAN NOT BE RECOVERED)")]
        Clear {
            #[arg(short, long)]
            project: bool,
        },
        #[command(about = "Change the status of the task (id) to COMPLETED")]
        Done {
            id: u8,

            #[arg(short, long)]
            project: bool,
        },
        #[command(about = "start the task (id)")]
        Start {
            id: u8,
        },
        #[command(about = "stop the task (id)")]
        Stop {
            id: u8,
        },
        #[command(about = "Show some blahblahblah from the author aka me")]
        Info {

        },
    }

    impl Command {
        pub fn destroy_project(conn: &Connection, id: u8) -> String {
            let mut tasks = fetch_task(conn).unwrap();
            tasks.iter_mut().for_each(|task| {
                if let Some(pro_id) = task.project_id {
                    if pro_id == id as i32 {
                        let _ = delete_task(&conn, task.id);
                    } 
                }
            });
            match delete_project(conn, id as i32) {
                Ok(_) => String::from("Project Destroyed."),
                Err(_) => String::from("No Matches."),
            }

        }
        pub fn complete_project(conn: &Connection, id: u8) -> String {
            let mut tasks = fetch_task(conn).unwrap();
            tasks.iter_mut().for_each(|task| {
                if let Some(pro_id) = task.project_id {
                    if pro_id == id as i32 {
                        let _ = Self::change(&conn, task.id as u8, 1);
                    } 
                }
            });
            match fetch_project_by_index(conn, id as i32) {
                Ok(mut pro) => {
                    pro.project_done();
                    String::from("Project Done.")
                },
                Err(_) => String::from("No Matches.")
            }
        }
        pub fn config(
            path: Option<String>, 
            date_format: Option<usize>,
        ) -> String {
            let mut modified_flag = false;
            let mut config = CONFIG.lock().unwrap();
            if let Some(p) = path {
                config.path = p.clone();
                modified_flag = true;
            }
            if let Some(f) = date_format {
                if f > 0 && f <= 11 { 
                    config.date_format= f;
                    modified_flag = true;
                } 
            }
            if modified_flag {
                config.config_write();
                String::from("Configuration Done.")
            } else {
                String::from("Nothing Changed.")
            }
        }

        pub fn init(conn: &Connection) -> String {
            let _ = create_table(&conn);
            let _ = create_project_table(&conn);
            let _ = create_trigger(&conn);
            String::from("Database Initialized.")
        }

        pub fn add(conn: &Connection, name: String, due: Option<String>, 
            scheduled: Option<String>, project_id: Option<i32>) -> String {
            let mut task = Task::new();
            let pro = if let Some(id) = project_id {
                Some(fetch_project_by_index(conn, id).unwrap())
            } else {
                None
            }; 
            task.set_name(name);
            task.set_date(due, 0);
            task.set_date(scheduled, 1);
            task.set_project(pro);
            task.verify();
            let _ = insert_task(&conn, &task);
            String::from("Task Added.")
        }

        pub fn add_project(conn: &Connection, name: String, 
            deadline: Option<String>, description: Option<String>) -> String {
            let mut pro = Project::new();
            pro.set_name(name);
            pro.set_deadline(deadline);
            pro.set_description(description);
            let _ = insert_project(&conn, &pro);
            String::from("Project Added.")
        }

        // pub fn show_task_by_project(conn: &Connection, project_id: u8, state_words: &mut Vec<StateWord>) -> String {
        //     let mut tasks = fetch_task(&conn).unwrap();
        //     if tasks.is_empty() {
        //         String::from("No Matches.")
        //     } else {
        //         task::sort_tasks(&mut tasks);
        //         Table::new(tasks.iter().filter_map(|task|{
        //             if let Some(p_id) = task.project_id {
        //                 if p_id == project_id as i32 {
        //                     state_words.push(task.get_state_word());
        //                     Some(task)
        //                 } else {
        //                     None
        //                 } 
        //             } else {
        //                 None
        //             }
        //         })).with(Style::empty()).to_string()
        //     }
        // }

        pub fn show(conn: &Connection, id: Option<u8>, a: bool, 
            filter: Option<Filter>, state_words: &mut Vec<StateWord>) -> String {
            if let Some(id) = id {
                // let task = fetch_task_by_index(&conn, id as i32).unwrap();
                match fetch_task_by_index(&conn, id as i32) {
                    Ok(task) => {
                        state_words.push(task.get_state_word());
                        Table::new(&vec![task])
                            .with(Style::empty())
                            .to_string()
                    },
                    Err(_) => {
                        String::from("No Matches.")
                    }
                }
            } else if !a {
                let mut tasks = fetch_task(&conn).unwrap();
                if tasks.is_empty() {
                    String::from("No Matches.")
                } else {
                    task::sort_tasks(&mut tasks);
                    if let Some(filter) = filter {
                        Table::new(tasks.iter().filter_map(|task|{
                            if task.filtered(&filter) {
                                state_words.push(task.get_state_word());
                                Some(task)
                            } else {
                                None
                            }
                        })).with(Style::empty()).to_string()
                    } else {
                        Table::new(tasks.iter().filter_map(|task|{
                            match task.status {
                                TaskStatus::Pending => {
                                    state_words.push(task.get_state_word());
                                    Some(task)
                                },
                                _ => None,
                            }
                        })).with(Style::empty()).to_string()
                    }
                }
            } else {
                let mut tasks = fetch_task(&conn).unwrap();
                if tasks.is_empty() {
                    String::from("No Match.")
                } else {
                    task::sort_tasks(&mut tasks);
                    for task in &tasks {
                        state_words.push(task.get_state_word());
                    }
                    Table::new(&tasks)
                        .with(Style::empty())
                        .to_string()
                }

            }
        }
        pub fn show_with_project(conn: &Connection, id: Option<u8>,
            filter: Option<Filter>, state_words: &mut Vec<StateWord>) -> String {
            if let Some(id) = id {
                let mut tasks = fetch_task(&conn).unwrap();
                if tasks.is_empty() {
                    String::from("No Match.")
                } else {
                    task::sort_tasks(&mut tasks);
                    if let Some(f) = filter {
                        Table::new(tasks.iter().filter_map(|task|{
                            if task.filtered(&f) && 
                            task.project_id == Some(id as i32) {
                                state_words.push(task.get_state_word());
                                Some(task)
                            } else {
                                None
                            }
                        })).with(Style::empty()).to_string()
                    } else {
                        Table::new(tasks.iter().filter_map(|task|{
                            if task.project_id == Some(id as i32) {
                                state_words.push(task.get_state_word());
                                Some(task)
                            } else {
                                None
                            }
                        })).with(Style::empty()).to_string()
                    }
                }
            } else {
                let projects = fetch_project(&conn).unwrap();
                if projects.is_empty() {
                    String::from("No Matches.")
                } else {
                    projects.iter().for_each(|_|state_words.push(0));
                    Table::new(&projects).with(Style::empty()).to_string()
                }
            }
        }

        // pub fn show_all(conn: &Connection, state_words: &mut Vec<StateWord>) -> String {
        //     let mut tasks = fetch_task(&conn).unwrap();
        //     if tasks.is_empty() {
        //         String::from("No Match.")
        //     } else {
        //         task::sort_tasks(&mut tasks);
        //         for task in &tasks {
        //             state_words.push(task.get_state_word());
        //         }
        //         Table::new(&tasks)
        //             .with(Style::empty())
        //             .to_string()
        //     }
        // }

        // pub fn show_projects(conn: &Connection, state_words: &mut Vec<StateWord> ) -> String {
        //     let pros= fetch_project(&conn).unwrap();
        //     if pros.is_empty() {
        //         String::from("No Match.")
        //     } else {
        //         for _ in &pros {
        //             state_words.push(0);
        //         }
        //         Table::new(&pros)
        //             .with(Style::empty())
        //             .to_string()
        //     }
        // }


        pub fn clear(conn: &Connection, project: bool) -> String {
            println!("WARNING! The operation will clear all data in database! (Y/N)?");
            let mut make_sure = String::new();
            match io::stdin().read_line(&mut make_sure) {
                Ok(_) => {
                    if make_sure.chars().nth(0).unwrap() == 'Y' {
                        let _ = if !project{
                            delete_all(&conn)
                        } else {
                            delete_all_projects(&conn)
                        }; 
                        String::from("Database Cleared.")
                    } else {
                        String::from("Operation Canceled.")
                    }
                },
                Err(_) => {
                    String::from("Operation Canceled.")
                },
            }
        }

        pub fn modify_project(
            conn: &Connection, id: u8, 
            name:Option<String>, 
            deadline: Option<String>, 
            description: Option<String>
        ) -> String {
            match fetch_project_by_index(&conn, id as i32) {
                Ok(mut pro) => {
                    if let Some(name) = name {
                        pro.set_name(name);
                    }
                    if let Some(deadline) = deadline {
                        if deadline == "" {
                            pro.set_deadline(None);
                        } else {
                            pro.set_deadline(Some(deadline));
                        }
                    }
                    if let Some(contents) = description {
                        if contents == "" {
                            pro.set_description(None);
                        } else {
                            pro.set_description(Some(contents));
                        }
                    }
                    let _ = update_project(conn, &pro).unwrap();
                    String::from("Project Modified.")
                },
                Err(_) => String::from("No Matches.")
            }
        }

        pub fn modify(
            conn: &Connection,
            id: u8, name:Option<String>,  due: Option<String>, 
            scheduled: Option<String>, project_id: Option<i32>
        ) -> String {
            match fetch_task_by_index(&conn, id as i32) {
                Ok(mut task) => {
                    if let Some(id) = project_id {
                        if id == 0 {
                            task.set_project(None);
                        } else {
                            let pro = fetch_project_by_index(conn, id).unwrap();
                            task.set_project(Some(pro));
                        }
                    }
                    if let Some(due) = due {
                        if due == "" {
                            task.set_date(None, 0);
                        } else {
                            task.set_date(Some(due), 0);
                        }
                    }
                    if let Some(scheduled) = scheduled {
                        if scheduled == "" {
                            task.set_date(None, 1);
                        } else {
                            task.set_date(Some(scheduled), 1);
                        }
                    }
                    if let Some(name) = name {
                        task.set_name(name);
                    }
                    task.verify();
                    let _ = update_task(&conn, &task).unwrap();
                    String::from("Task Modified.")
                },
                Err(_) => {
                    String::from("No Matches.")
                }, 
            }

        }

        pub fn change(conn: &Connection, id: u8, command_type: u8) -> String {
            let mut task = match fetch_task_by_index(&conn, id as i32) {
                Ok(p) => p,
                Err(_) => return String::from("No Matches"),
            };
            let mut res = String::from("");
            match command_type {
                // delete
                0 => {
                    task.delete();
                    res = String::from("Task Canceled.")
                },
                // done
                1 => {
                    task.done();
                    res = String::from("Task Completed.")
                },
                // start
                2 => {
                    task.start();
                    res = String::from("Task Start.")
                },
                3 => {
                    task.stop();
                    res = String::from("Task Stop.")
                }
                _ => {}, 
            }
            let _ = update_task(&conn, &task).unwrap();
            res
        }

        pub fn show_config() -> String {
            println!("Database directory: {}", &get_database_dir());
            let (_, date_format) = get_date_format();
            println!("Date format: {}", date_format);
            format!("You can change the configuration in {}, or use the config command.", &get_config_dir())
            // String::from("You can change the configuration in {}", CONFIG_DIR)
        }

        pub fn delele_project(conn: &Connection, id: u8) -> String {
            let mut tasks = fetch_task(conn).unwrap();
            tasks.iter_mut().for_each(|task| {
                if let Some(pro_id) = task.project_id {
                    if pro_id == id as i32 {
                        task.set_project(None);
                        let _ = update_task(&conn, &task);
                    }
                }
            });
            match delete_project(conn, id as i32) {
                Ok(_) => String::from("Project Deleted"),
                Err(_) => String::from("No Matches."),
            }

        }
    }
}
