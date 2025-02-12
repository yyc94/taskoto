pub mod taskoto {

    use clap::{self, Parser};
    use tabled::{Table, settings::Style};
    use ansi_term::Colour;
    use std::io;

    use crate::task::task::{self, Task, StateWord,  TaskStatus};
    use crate::parser::parser::{Cli, Command}; 
    use crate::database::database::*;
    use crate::Config;

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
            match command {
                Command::Config { path, date_format } => {
                    let mut modified_flag = false;
                    match path {
                        Some(p) => {
                            let mut config = Config::default();
                            config.path = p.clone();
                            config.config_write();
                            modified_flag = true;
                        },
                        None => {},
                    }
                    match date_format {
                        Some(f) => {
                            if f > 0 && f <= 11 { 
                                let mut config = Config::default();
                                config.date_format= f;
                                config.config_write();
                                modified_flag = true;
                            } 
                        },
                        None => {},
                    }
                    res = if modified_flag {
                        String::from("Configuration Done.")
                    } else {
                            String::from("Nothing Changed.")
                    };
                },
                Command::Init{  } => {
                    let _ = create_table(&conn);
                    res = String::from("Database Initialized.");
                }
                Command::Add {
                    name, 
                    due, 
                    scheduled, 
                    project } => {
                    let mut task = Task::new();
                    task.set_name(name);
                    task.set_date(due, 0);
                    task.set_date(scheduled, 1);
                    task.set_project(project);
                    task.verify();
                    let _ = insert_task(&conn, &task);
                    res = String::from("Task Added.");
                },
                Command::Show { id, filter } => {
                    if let Some(id) = id {
                        // let task = fetch_task_by_index(&conn, id as i32).unwrap();
                        res = match fetch_task_by_index(&conn, id as i32) {
                            Ok(task) => {
                                state_words.push(task.get_state_word());
                                Table::new(&vec![task])
                                    .with(Style::empty())
                                    .to_string()
                            },
                            Err(_) => {
                                String::from("No Matches.")
                            }
                        };
                        // res = Table::new(&vec![task]).to_string();
                    } else {
                        let mut tasks = fetch_task(&conn).unwrap();
                        if tasks.is_empty() {
                            res = String::from("No Matches.")
                        } else {
                            task::sort_tasks(&mut tasks);
                            if let Some(filter) = filter {
                                res = Table::new(tasks.iter().filter_map(|task|{
                                    if task.filtered(&filter) {
                                        state_words.push(task.get_state_word());
                                        Some(task)
                                    } else {
                                        None
                                    }
                                })).with(Style::empty()).to_string();
                            } else {
                                res = Table::new(tasks.iter().filter_map(|task|{
                                    match task.status {
                                        TaskStatus::Pending => {
                                            state_words.push(task.get_state_word());
                                            Some(task)
                                        },
                                        _ => None,
                                    }
                                })).with(Style::empty()).to_string();
                            }
                        }
                    }
                },
                Command::Modify {
                    id, 
                    name,
                    due,
                    scheduled, 
                    project } => {
                    // let mut task = fetch_task_by_index(&conn, id as i32).unwrap();
                    res = match fetch_task_by_index(&conn, id as i32) {
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
                        Err(_) => String::from("No Matches."), 
                    }
                },
                Command::Delete { id } => {
                    res = match fetch_task_by_index(&conn, id as i32) {
                        Ok(mut task) => {
                            task.delete();
                            let _ = update_task(&conn, &task).unwrap();
                            String::from("Task Canceled.")
                        },
                        Err(_) => String::from("No Matches."),
                    }
                    // let mut task = fetch_task_by_index(&conn, id as i32).unwrap();
                    // task.delete();
                    // let _ = update_task(&conn, &task).unwrap();
                    // res = String::from("Task Canceled");
                },
                Command::Destroy { id } => {
                    res = match delete_task(&conn, id as i32) {
                        Ok(_) => String::from("Task Destroyed."),
                        Err(_) => String::from("No Matches."),
                    }
                    // let _ = delete_task(&conn, id as i32).unwrap();
                    // res = String::from("Task Destroyed");
                },
                Command::ShowAll {  } => {
                    let mut tasks = fetch_task(&conn).unwrap();
                    if tasks.is_empty() {
                        res = String::from("No Match.");
                    } else {
                        task::sort_tasks(&mut tasks);
                        for task in &tasks {
                            state_words.push(task.get_state_word());
                        }
                        res = Table::new(&tasks)
                            .with(Style::empty())
                            .to_string();
                    }
                }
                Command::Done { id } => {
                    res = match fetch_task_by_index(&conn, id as i32) {
                        Ok(mut task) => {
                            task.done();
                            let _ = update_task(&conn, &task).unwrap();
                            String::from("Task Done.")
                        },
                        Err(_) => String::from("No Matches."),
                    }
                    // let mut task = fetch_task_by_index(&conn, id as i32).unwrap();
                    // task.done();
                    // let _ = update_task(&conn, &task).unwrap();
                    // res = String::from("Task Done");
                },
                Command::Start { id } => {
                    res = match fetch_task_by_index(&conn, id as i32) {
                        Ok(mut task) => {
                            task.start();
                            let _ = update_task(&conn, &task).unwrap();
                            String::from("Task Start.")
                        },
                        Err(_) => String::from("No Matches.")
                    }
                    // let mut task = fetch_task_by_index(&conn, id as i32).unwrap();
                    // task.start();
                    // let _ = update_task(&conn, &task).unwrap();
                    // res = String::from("Task Start");
                },
                Command::Stop { id } => {
                    res = match fetch_task_by_index(&conn, id as i32) {
                        Ok(mut task) => {
                            task.stop();
                            let _ = update_task(&conn, &task).unwrap();
                            String::from("Task Stop.")
                        },
                        Err(_) => String::from("No Matches.")
                    }
                    // let mut task = fetch_task_by_index(&conn, id as i32).unwrap();
                    // task.end();
                    // let _ = update_task(&conn, &task).unwrap();
                    // res = String::from("Task End");
                },
                Command::Clear {  } => {
                    println!("WARNING! The operation will clear all data in database! (Y/N)?");
                    let mut make_sure = String::new();
                    match io::stdin().read_line(&mut make_sure) {
                        Ok(_) => {
                            if make_sure.chars().nth(0).unwrap() == 'Y' {
                                let _ = delete_all(&conn);
                                res = String::from("Database Cleared.");
                            } else {
                                res = String::from("Operation Canceled.")
                            }
                        },
                        Err(_) => {
                            res = String::from("Operation Canceled.")
                        },
                    }
                },
                Command::Info {  } => {
                    res = String::from("I have too much to say, but I can't fit a line.")
                }
            }
        } else {
            let mut tasks = fetch_task(&conn).unwrap();
            if tasks.is_empty() {
                res = String::from("No Matches.");
            } else {
                task::sort_tasks(&mut tasks);
                res = Table::new(tasks.iter().filter_map(|task|{
                    match task.status {
                        TaskStatus::Pending => {
                            state_words.push(task.get_state_word());
                            Some(task)
                        },
                        _ => None,
                    }
                })).with(Style::empty()).to_string();
            }
        }
        // println!("{}", res);
        show_table(&res, state_words);
        disconnect_to_db(conn);
    }

}
