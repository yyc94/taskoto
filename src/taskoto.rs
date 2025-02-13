pub mod taskoto {

    use clap::{self, Parser};
    use tabled::{Table, settings::Style};
    use ansi_term::Colour;
    use std::io;

    use crate::task::task::{self, Task, StateWord, TaskStatus, Filter};
    use crate::parser::parser::{Cli, Command}; 
    use crate::database::database::*;
    use rusqlite::Connection;
    use crate::*;
    use crate::sync::sync::*;

    fn set_style(s: &str, state: StateWord, flag: bool) {
        let color = if state & 1 != 0 {
            Colour::Red
        } else {
            Colour::White
        };
        let style = if state & (1 << 1) != 0 {
            color.blink()
        } else if state & (1 << 2) != 0 {
            color.strikethrough()
        } else if state & (1 << 3) != 0 {
            color.dimmed()
        } else if state & (1 << 4) != 0 {
            color.strikethrough()
        } else {
            color.normal()
        };
        if flag {
            // style.on(Colour::Black);
            println!("{}", style.on(Colour::Black).paint(s));
        } else {
            // style.on(Colour::RGB(169, 169, 169));
            // style.on(Colour::Green);
            println!("{}", style.on(Colour::RGB(32, 32, 32)).paint(s));
        }
    }
    fn show_table(table: &str, states: Vec<StateWord>) {
        let rows: Vec<&str> = table.split("\n").collect();
        let mut flag: bool = false;
        let mut is_header = true;
        let n = states.len();
        for (i, &s) in rows.iter().enumerate() {
            if i > n {
                set_style(s, 0, true);
                continue;
            }
            if is_header {
                is_header = false;
                println!("{}", Colour::White.bold().underline().paint(s));
            } else if flag {
                flag = false;
                set_style(s, states[i - 1], flag);
            } else {
                flag = true;
                set_style(s, states[i - 1], flag);
            }
        }
    }
    pub fn taskoto_run() {
        let args = Cli::parse(); 
        let mut state_words = Vec::new();
        let res;
        let conn = connect_to_db().unwrap();

        if let Some(command) = args.command {
            res = match command {
                Command::Config {
                    user_name, 
                    email, 
                    path,
                    date_format,
                    sync,
                    sync_url
                } => {
                    command_config(user_name, email, path, date_format, sync, sync_url)
                },
                Command::ShowConfig {  } => {
                    "hh".to_string()
                },
                Command::Init{  } => {
                    command_init(&conn)
                }
                Command::Add {
                    name, 
                    due, 
                    scheduled, 
                    project } => {
                    command_add(&conn, name, due, scheduled, project)
                },
                Command::Show { id, filter } => {
                    command_show(&conn, id, filter, &mut state_words)
                },
                Command::Modify {
                    id, 
                    name,
                    due,
                    scheduled, 
                    project } => {
                    command_modify(&conn, id, name, due, scheduled, project)        // let mut task = fetch_task_by_index(&conn, id as i32).unwrap();
                },
                Command::Delete { id } => {
                    command_change(&conn, id, 0)
                },
                Command::Destroy { id } => {
                    match delete_task(&conn, id as i32) {
                        Ok(_) => String::from("Task Destroyed."),
                        Err(_) => String::from("No Matches."),
                    }
                },
                Command::ShowAll {  } => {
                    command_show_all(&conn, &mut state_words)
                }
                Command::Done { id } => {
                    command_change(&conn, id, 1)
                },
                Command::Start { id } => {
                    command_change(&conn, id, 2)
                },
                Command::Stop { id } => {
                    command_change(&conn, id, 3)
                },
                Command::Clear {  } => {
                    command_clear(&conn)
                },
                Command::Info {  } => {
                    String::from("I have too much to say, but I can't fit a line.")
                }
            }
        } else {
            res = command_show(&conn,None, None, &mut state_words);
        }
        disconnect_to_db(conn);
        // println!("{}", res);
        show_table(&res, state_words);
    }

    fn command_config(
        user_name: Option<String>,
        email: Option<String>,
        path: Option<String>, 
        date_format: Option<usize>,
        sync: Option<bool>,
        sync_url: Option<String>,
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
        if let Some(u) = user_name {
            config.user_name = u.clone();
            modified_flag = true;
        }
        if let Some(e) = email {
            config.email = e.clone();
            modified_flag = true;
        }
        if let Some(sync) = sync {
            config.sync = sync;
            modified_flag = true;
        }
        if let Some(s) = sync_url {
            config.sync_url = s.clone();
            modified_flag = true;
        }
        if modified_flag {
            config.config_write();
            String::from("Configuration Done.")
        } else {
            String::from("Nothing Changed.")
        }
    }

    fn command_init(conn: &Connection) -> String {
        let _ = create_table(&conn);
        if is_sync() {
            let _ = init_repo();
        }
        String::from("Database Initialized.")
    }

    fn command_add(conn: &Connection, name: String, due: Option<String>, 
        scheduled: Option<String>, project: Option<String>) -> String {
        let mut task = Task::new();
        task.set_name(name);
        task.set_date(due, 0);
        task.set_date(scheduled, 1);
        task.set_project(project);
        task.verify();
        let _ = insert_task(&conn, &task);
        String::from("Task Added.")
    }

    fn command_show(conn: &Connection, id: Option<u8>, filter: Option<Filter>, state_words: &mut Vec<StateWord>) -> String {
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
            // res = Table::new(&vec![task]).to_string();
        } else {
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
        }
    }
    fn command_show_all(conn: &Connection, state_words: &mut Vec<StateWord>) -> String {
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

    fn command_clear(conn: &Connection) -> String {
        println!("WARNING! The operation will clear all data in database! (Y/N)?");
        let mut make_sure = String::new();
        match io::stdin().read_line(&mut make_sure) {
            Ok(_) => {
                if make_sure.chars().nth(0).unwrap() == 'Y' {
                    let _ = delete_all(&conn);
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

    fn command_modify(
        conn: &Connection,
        id: u8, name:Option<String>,  due: Option<String>, 
        scheduled: Option<String>, project: Option<String>
    ) -> String {
        match fetch_task_by_index(&conn, id as i32) {
            Ok(mut task) => {
                if let Some(project) = project {
                    if project == "" {
                        task.set_project(None);
                    } else {
                        task.set_project(Some(project));
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

    fn command_change(conn: &Connection, id: u8, command_type: u8) -> String {
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

}
