pub mod taskoto {

    use clap::{self, Parser};
    use ansi_term::Colour;

    use crate::task::task::StateWord;
    use crate::parser::parser::{Cli, Command}; 
    use crate::database::database::*;

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
                    path,
                    date_format,
                } => {
                    Command::config(path, date_format)
                },
                Command::ShowConfig {  } => {
                    Command::show_config()
                },
                Command::Init{  } => {
                    Command::init(&conn)
                }
                Command::Add {
                    name, 
                    due, 
                    scheduled, 
                    project_id } => {
                    Command::add(&conn, name, due, scheduled, project_id)
                },
                Command::AddProject { name, deadline, description } => {
                    Command::add_project(&conn, name, deadline, description)
                },
                Command::Show { id, filter , project, all} => {
                    if project {
                        Command::show_with_project(&conn, id, filter, &mut state_words)
                    } else {
                        Command::show(&conn, id, all, filter, &mut state_words, )
                    }
                    // if let Some(id) = id {
                    //     if !project {
                    //     } else {
                    //         Command::show_task_by_project(&conn, id, &mut state_words)
                    //     }
                    // } else {
                    //     if project {
                    //         Command::show_projects(&conn, &mut state_words)
                    //     } else if !all {
                    //         Command::show(&conn, id, filter, &mut state_words)
                    //     } else {
                    //         Command::show_all(&conn, &mut state_words)
                    //     }
                    // }
                },
                Command::Modify {
                    id, 
                    name,
                    due,
                    scheduled, 
                    project_id } => {
                    Command::modify(&conn, id, name, due, scheduled, project_id)        
                    // let mut task = fetch_task_by_index(&conn, id as i32).unwrap();
                },
                Command::Delete { id , project} => {
                    if project {
                        Command::delele_project(&conn, id)
                    } else {
                        Command::change(&conn, id, 0)
                    }
                },
                Command::Destroy { id , project} => {
                    if project {
                        // command_destroy_project(&conn, id)
                        Command::destroy_project(&conn, id)
                    } else {
                        match delete_task(&conn, id as i32) {
                            Ok(_) => String::from("Task Destroyed."),
                            Err(_) => String::from("No Matches."),
                        }
                    } 
                },
                Command::Done { id , project} => {
                    if project {
                        Command::complete_project(&conn, id)
                    } else {
                        Command::change(&conn, id, 1)
                    }
                },
                Command::Start { id } => {
                    Command::change(&conn, id, 2)
                },
                Command::Stop { id } => {
                    Command::change(&conn, id, 3)
                },
                Command::Info {  } => {
                    String::from("I have too much to say, but I can't fit a line.")
                },
                Command::ModifyProject { 
                    id, 
                    name, 
                    deadline, 
                    description } => {
                    Command::modify_project(
                        &conn, id, name, deadline, description)
                },
                Command::Clear { project } => {
                    Command::clear(&conn, project)
                },
            }
        } else {
            res = Command::show(&conn, None, false, None, &mut state_words);
        }
        disconnect_to_db(conn);
        // println!("{}", res);
        show_table(&res, state_words);
    }

    // fn command_config(
    //     path: Option<String>, 
    //     date_format: Option<usize>,
    // ) -> String {
    //     let mut modified_flag = false;
    //     let mut config = CONFIG.lock().unwrap();
    //     if let Some(p) = path {
    //         config.path = p.clone();
    //         modified_flag = true;
    //     }
    //     if let Some(f) = date_format {
    //         if f > 0 && f <= 11 { 
    //             config.date_format= f;
    //             modified_flag = true;
    //         } 
    //     }
    //     if modified_flag {
    //         config.config_write();
    //         String::from("Configuration Done.")
    //     } else {
    //         String::from("Nothing Changed.")
    //     }
    // }

    // fn command_init(conn: &Connection) -> String {
    //     let _ = create_table(&conn);
    //     let _ = create_project_table(&conn);
    //     let _ = create_trigger(&conn);
    //     String::from("Database Initialized.")
    // }

    // fn command_add(conn: &Connection, name: String, due: Option<String>, 
    //     scheduled: Option<String>, project_id: Option<i32>) -> String {
    //     let mut task = Task::new();
    //     let pro = if let Some(id) = project_id {
    //         Some(fetch_project_by_index(conn, id).unwrap())
    //     } else {
    //         None
    //     }; 
    //     task.set_name(name);
    //     task.set_date(due, 0);
    //     task.set_date(scheduled, 1);
    //     task.set_project(pro);
    //     task.verify();
    //     let _ = insert_task(&conn, &task);
    //     String::from("Task Added.")
    // }

    // fn command_add_project(conn: &Connection, name: String, 
    //     deadline: Option<String>, description: Option<String>) -> String {
    //     let mut pro = Project::new();
    //     pro.set_name(name);
    //     pro.set_deadline(deadline);
    //     pro.set_description(description);
    //     let _ = insert_project(&conn, &pro);
    //     String::from("Project Added.")
    // }

    // fn command_show_task_by_project(conn: &Connection, project_id: u8, state_words: &mut Vec<StateWord>) -> String {
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

    // fn command_show(conn: &Connection, id: Option<u8>, 
    //     filter: Option<Filter>, state_words: &mut Vec<StateWord>) -> String {
    //     if let Some(id) = id {
    //         // let task = fetch_task_by_index(&conn, id as i32).unwrap();
    //         match fetch_task_by_index(&conn, id as i32) {
    //             Ok(task) => {
    //                 state_words.push(task.get_state_word());
    //                 Table::new(&vec![task])
    //                     .with(Style::empty())
    //                     .to_string()
    //             },
    //             Err(_) => {
    //                 String::from("No Matches.")
    //             }
    //         }
    //         // res = Table::new(&vec![task]).to_string();
    //     } else {
    //         let mut tasks = fetch_task(&conn).unwrap();
    //         if tasks.is_empty() {
    //             String::from("No Matches.")
    //         } else {
    //             task::sort_tasks(&mut tasks);
    //             if let Some(filter) = filter {
    //                 Table::new(tasks.iter().filter_map(|task|{
    //                     if task.filtered(&filter) {
    //                         state_words.push(task.get_state_word());
    //                         Some(task)
    //                     } else {
    //                         None
    //                     }
    //                 })).with(Style::empty()).to_string()
    //             } else {
    //                 Table::new(tasks.iter().filter_map(|task|{
    //                     match task.status {
    //                         TaskStatus::Pending => {
    //                             state_words.push(task.get_state_word());
    //                             Some(task)
    //                         },
    //                         _ => None,
    //                     }
    //                 })).with(Style::empty()).to_string()
    //             }
    //         }
    //     }
    // }

    // fn command_show_all(conn: &Connection, state_words: &mut Vec<StateWord>) -> String {
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
    //
    // }

    // fn command_show_projects(conn: &Connection, state_words: &mut Vec<StateWord> ) -> String {
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
    //
    // }


    // fn command_clear(conn: &Connection, project: bool) -> String {
    //     println!("WARNING! The operation will clear all data in database! (Y/N)?");
    //     let mut make_sure = String::new();
    //     match io::stdin().read_line(&mut make_sure) {
    //         Ok(_) => {
    //             if make_sure.chars().nth(0).unwrap() == 'Y' {
    //                 let _ = if !project{
    //                     delete_all(&conn)
    //                 } else {
    //                     delete_all_projects(&conn)
    //                 }; 
    //                 String::from("Database Cleared.")
    //             } else {
    //                 String::from("Operation Canceled.")
    //             }
    //         },
    //         Err(_) => {
    //             String::from("Operation Canceled.")
    //         },
    //     }
    // }

    // fn command_modify_project(
    //     conn: &Connection, id: u8, 
    //     name:Option<String>, 
    //     deadline: Option<String>, 
    //     description: Option<String>
    // ) -> String {
    //     match fetch_project_by_index(&conn, id as i32) {
    //         Ok(mut pro) => {
    //             if let Some(name) = name {
    //                 pro.set_name(name);
    //             }
    //             if let Some(deadline) = deadline {
    //                 if deadline == "" {
    //                     pro.set_deadline(None);
    //                 } else {
    //                     pro.set_deadline(Some(deadline));
    //                 }
    //             }
    //             if let Some(contents) = description {
    //                 if contents == "" {
    //                     pro.set_description(None);
    //                 } else {
    //                     pro.set_description(Some(contents));
    //                 }
    //             }
    //             let _ = update_project(conn, &pro).unwrap();
    //             String::from("Project Modified.")
    //         },
    //         Err(_) => String::from("No Matches.")
    //     }
    // }

    // fn command_modify(
    //     conn: &Connection,
    //     id: u8, name:Option<String>,  due: Option<String>, 
    //     scheduled: Option<String>, project_id: Option<i32>
    // ) -> String {
    //     match fetch_task_by_index(&conn, id as i32) {
    //         Ok(mut task) => {
    //             if let Some(id) = project_id {
    //                 if id == 0 {
    //                     task.set_project(None);
    //                 } else {
    //                     let pro = fetch_project_by_index(conn, id).unwrap();
    //                     task.set_project(Some(pro));
    //                 }
    //             }
    //             if let Some(due) = due {
    //                 if due == "" {
    //                     task.set_date(None, 0);
    //                 } else {
    //                     task.set_date(Some(due), 0);
    //                 }
    //             }
    //             if let Some(scheduled) = scheduled {
    //                 if scheduled == "" {
    //                     task.set_date(None, 1);
    //                 } else {
    //                     task.set_date(Some(scheduled), 1);
    //                 }
    //             }
    //             if let Some(name) = name {
    //                 task.set_name(name);
    //             }
    //             task.verify();
    //             let _ = update_task(&conn, &task).unwrap();
    //             String::from("Task Modified.")
    //         },
    //         Err(_) => {
    //             String::from("No Matches.")
    //         }, 
    //     }
    //
    // }

    // fn command_change(conn: &Connection, id: u8, command_type: u8) -> String {
    //     let mut task = match fetch_task_by_index(&conn, id as i32) {
    //         Ok(p) => p,
    //         Err(_) => return String::from("No Matches"),
    //     };
    //     let mut res = String::from("");
    //     match command_type {
    //         // delete
    //         0 => {
    //             task.delete();
    //             res = String::from("Task Canceled.")
    //         },
    //         // done
    //         1 => {
    //             task.done();
    //             res = String::from("Task Completed.")
    //         },
    //         // start
    //         2 => {
    //             task.start();
    //             res = String::from("Task Start.")
    //         },
    //         3 => {
    //             task.stop();
    //             res = String::from("Task Stop.")
    //         }
    //         _ => {}, 
    //     }
    //     let _ = update_task(&conn, &task).unwrap();
    //     res
    // }

    // fn command_show_config() -> String {
    //     println!("Database directory: {}", &get_database_dir());
    //     let (_, date_format) = get_date_format();
    //     println!("Date format: {}", date_format);
    //     format!("You can change the configuration in {}, or use the config command.", &get_config_dir())
    //     // String::from("You can change the configuration in {}", CONFIG_DIR)
    // }

    // fn command_delele_project(conn: &Connection, id: u8) -> String {
    //     let mut tasks = fetch_task(conn).unwrap();
    //     tasks.iter_mut().for_each(|task| {
    //         if let Some(pro_id) = task.project_id {
    //             if pro_id == id as i32 {
    //                 task.set_project(None);
    //                 let _ = update_task(&conn, &task);
    //             }
    //         }
    //     });
    //     match delete_project(conn, id as i32) {
    //         Ok(_) => String::from("Project Deleted"),
    //         Err(_) => String::from("No Matches."),
    //     }
    //
    // }

    // fn command_complete_project(conn: &Connection, id: u8) -> String {
    //     let mut tasks = fetch_task(conn).unwrap();
    //     tasks.iter_mut().for_each(|task| {
    //         if let Some(pro_id) = task.project_id {
    //             if pro_id == id as i32 {
    //                 let _ = command_change(&conn, task.id as u8, 1);
    //             } 
    //         }
    //     });
    //     match fetch_project_by_index(conn, id as i32) {
    //         Ok(mut pro) => {
    //             pro.project_done();
    //             String::from("Project Done.")
    //         },
    //         Err(_) => String::from("No Matches.")
    //     }
    // }

    // fn command_destroy_project(conn: &Connection, id: u8) -> String {
    //     let mut tasks = fetch_task(conn).unwrap();
    //     tasks.iter_mut().for_each(|task| {
    //         if let Some(pro_id) = task.project_id {
    //             if pro_id == id as i32 {
    //                 let _ = delete_task(&conn, task.id);
    //             } 
    //         }
    //     });
    //     match delete_project(conn, id as i32) {
    //         Ok(_) => String::from("Project Destroyed."),
    //         Err(_) => String::from("No Matches."),
    //     }
    //
    // }
}
