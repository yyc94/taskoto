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
                } => Command::config(path, date_format),
                Command::ShowConfig {  } => Command::show_config(),
                Command::Init{  } => Command::init(&conn),
                Command::Add {
                    name, 
                    due, 
                    scheduled, 
                    project_id 
                } => Command::add(&conn, name, due, scheduled, project_id),
                Command::AddProject {
                    name, 
                    deadline, 
                    description 
                } => Command::add_project(&conn, name, deadline, description),
                Command::Show { id, filter , project, all} => {
                    if project {
                        Command::show_with_project(&conn, id, filter, &mut state_words)
                    } else {
                            Command::show(&conn, id, all, filter, &mut state_words, )
                    }
                },
                Command::ShowDetail { id } => Command::show_details(&conn, id),
                Command::Modify {
                    id, 
                    name,
                    due,
                    scheduled, 
                    project_id 
                } => Command::modify(&conn, id, name, due, scheduled, project_id),
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
                Command::Start { id } => Command::change(&conn, id, 2),
                Command::Stop { id } => Command::change(&conn, id, 3),
                Command::Info {  } => String::from("I have too much to say, but I can't fit a line."),
                Command::ModifyProject { 
                    id, 
                    name, 
                    deadline, 
                    description } => Command::modify_project(&conn, id, name, deadline, description),
                Command::Clear { project } => Command::clear(&conn, project),
            }
        } else {
            res = Command::show(&conn, None, false, None, &mut state_words);
        }
        disconnect_to_db(conn);
        // println!("{}", res);
        show_table(&res, state_words);
    }
}
