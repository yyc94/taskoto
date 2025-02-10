mod test;
mod task;
mod database;
mod parser;

use parser::parser::*;
use clap::Parser;
use task::task::{Task, StateWord, sort_tasks};
use database::database::*;
use tabled::{Table, settings::Style};
use std::io;
use ansi_term::Colour;

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

fn main() {
    let conn = connect_to_db().unwrap();

    let res;

    let args = Cli::parse(); 
    let mut state_words = Vec::new();

    if let Some(command) = args.command {
        match command {
            Command::Init{} => {
                let _ = create_table(&conn);
                res = String::from("Database Initialized");
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
                    let task = fetch_task_by_index(&conn, id as i32).unwrap();
                    state_words.push(task.get_state_word());
                    res = Table::new(&vec![task]).to_string();
                } else {
                    let tasks = fetch_task(&conn).unwrap();
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
                                task::task::TaskStatus::Pending => {
                                    state_words.push(task.get_state_word());
                                    Some(task)
                                },
                                _ => None,
                            }
                        })).with(Style::empty()).to_string();
                    }
                }
            },
            Command::Modify {
                id, 
                name,
                due,
                scheduled, 
                project } => {
                let mut task = fetch_task_by_index(&conn, id as i32).unwrap();
                task.set_project(project);
                task.set_date(due, 0);
                task.set_date(scheduled, 1);
                if let Some(name) = name {
                    task.set_name(name);
                }
                task.verify();
                let _ = update_task(&conn, &task).unwrap();
                res = String::from("Task Modified");
            },
            Command::Delete { id } => {
                let mut task = fetch_task_by_index(&conn, id as i32).unwrap();
                task.delete();
                let _ = update_task(&conn, &task).unwrap();
                res = String::from("Task Canceled");
            },
            Command::Destroy { id } => {
                let _ = delete_task(&conn, id as i32).unwrap();
                res = String::from("Task Destroyed");
            },
            Command::ShowAll {  } => {
                let mut tasks = fetch_task(&conn).unwrap();
                sort_tasks(&mut tasks);
                for task in &tasks {
                    state_words.push(task.get_state_word());
                }
                res = Table::new(&tasks).with(Style::empty()).to_string();
            }
            Command::Done { id } => {
                let mut task = fetch_task_by_index(&conn, id as i32).unwrap();
                task.done();
                let _ = update_task(&conn, &task).unwrap();
                res = String::from("Task Done");
            },
            Command::Start { id } => {
                let mut task = fetch_task_by_index(&conn, id as i32).unwrap();
                task.start();
                let _ = update_task(&conn, &task).unwrap();
                res = String::from("Task Start");
            },
            Command::End { id } => {
                let mut task = fetch_task_by_index(&conn, id as i32).unwrap();
                task.end();
                let _ = update_task(&conn, &task).unwrap();
                res = String::from("Task End");
            },
            Command::Clear {  } => {
                println!("WARNING! The operation will clear all data in database! (Y/N)?");
                let mut make_sure = String::new();
                match io::stdin().read_line(&mut make_sure) {
                    Ok(_) => {
                        if make_sure.chars().nth(0).unwrap() == 'Y' {
                            let _ = delete_all(&conn);
                            res = String::from("Database Cleared");
                        } else {
                            res = String::from("Operation Canceled")
                        }
                    },
                    Err(_) => {
                        res = String::from("Operation Canceled")
                    },
                }
            }
        }
    } else {
        let tasks = fetch_task(&conn).unwrap();
        res = Table::new(tasks.iter().filter_map(|task|{
            match task.status {
                task::task::TaskStatus::Pending => {
                    state_words.push(task.get_state_word());
                    Some(task)
                },
                _ => None,
            }
        })).with(Style::empty()).to_string();
    }
    // println!("{}", res);
    show_table(&res, state_words);
    disconnect_to_db(conn);
}
