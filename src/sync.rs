// pub mod sync {
//
//     use std::{
//         process::Command,
//         io::{self, Write},
//         fs::OpenOptions,
//     };
//     use chrono::Local;
//
//     use crate::*;
//
//     const REMOTE_NAME: &str = "origin";
//     const GIT_IGNORE: &str = ".gitignore";
//
//     pub fn init_repo() {
//         match run_git_command(&["init", &get_database_dir()]) {
//             Ok(output) => println!("Git init output:\n{}", output),
//             Err(e) => {
//                 println!("Error running git command: {}", e);
//                 return;
//             }, 
//         }
//
//         let mut gitignore = OpenOptions::new()
//             .write(true)
//             .append(true)
//             .create(true)
//             .open(&(get_database_dir() + GIT_IGNORE))
//             .unwrap();
//         gitignore.write_all(b"*\n!taskoto.db").unwrap();
//
//         match run_git_command(&["-C", &get_database_dir(),"add", "-A"]) {
//             Ok(output) => println!("Git add output:\n{}", output),
//             Err(e) => {
//                 println!("Error running git command: {}", e);
//                 return;
//             }, 
//         }
//
//         match run_git_command(&["-C", &get_database_dir(), "commit", "-m", "first commit"]) {
//             Ok(output) => println!("Git commit output:\n{}", output),
//             Err(e) => {
//                 println!("Error running git command: {}", e);
//                 return;
//             }, 
//         }
//
//         match run_git_command(&["-C", &get_database_dir(), "branch", "-M", "main"]) {
//             Ok(output) => println!("Git branch output:\n{}", output),
//             Err(e) => {
//                 println!("Error running git command: {}", e);
//                 return;
//             }, 
//         }
//
//         match run_git_command(&["-C", &get_database_dir(), "remote", "add", REMOTE_NAME, &get_sync_url()]) {
//             Ok(output) => println!("Git remote add output:\n{}", output),
//             Err(e) => {
//                 println!("Error running git command: {}", e);
//                 return;
//             }, 
//         }
//         match run_git_command(&["-C", &get_database_dir(), "push", "-u", REMOTE_NAME, "main"]) {
//             Ok(output) => println!("Git push output:\n{}", output),
//             Err(e) => {
//                 println!("Error running git command: {}", e);
//                 return;
//             }, 
//         }
//     } 
//
//     pub fn sync_push() {
//         match run_git_command(&["-C", &get_database_dir(),"add", "-A"]) {
//             Ok(output) => println!("Git add output:\n{}", output),
//             Err(e) => {
//                 println!("Error running git command: {}", e);
//                 return;
//             }, 
//         }
//         let now = Local::now().date_naive().to_string(); 
//
//         match run_git_command(&["-C", &get_database_dir(), "commit", "-m", &now]) {
//             Ok(output) => println!("Git commit output:\n{}", output),
//             Err(e) => {
//                 println!("Error running git command: {}", e);
//                 return;
//             }, 
//         }
//
//         match run_git_command(&["-C", &get_database_dir(), "push", REMOTE_NAME, "main"]) {
//             Ok(output) => println!("Git push output:\n{}", output),
//             Err(e) => {
//                 println!("Error running git command: {}", e);
//                 return;
//             }, 
//         }
//        
//
//     }
//
//     pub fn sync_pull() {
//         match run_git_command(&["-C", &get_database_dir(), "pull", REMOTE_NAME, "main"]) {
//             Ok(output) => println!("Git pull output:\n{}", output),
//             Err(e) => {
//                 println!("Error running git command: {}", e);
//             }, 
//         }
//
//     }
//
//     fn run_git_command(args: &[&str]) -> io::Result<String>{ 
//         let output = Command::new("git") 
//             .args(args) 
//             .output()?; 
//         if output.status.success() {
//             Ok(String::from_utf8_lossy(&output.stdout).to_string())
//         } else {
//             Err(io::Error::new(
//                 io::ErrorKind::Other,
//                 String::from_utf8_lossy(&output.stderr).to_string(),
//             ))
//         }
//     }
// }
