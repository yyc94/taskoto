pub mod parser {
    use clap::{Parser, Subcommand};
    use crate::task::task::Filter;

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Option<Command>,
    }
    #[derive(Subcommand)]
    pub enum Command {
        #[command(about = "Modify the configuration of the taskoto")]
        Config {
            #[arg(short, long)]
            path: Option<String>,

            #[arg(short, long)]
            date_format: Option<usize>,

            //...
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
            project: Option<String>,

        },
        #[group(multiple = false)]
        #[command(about = "Show the task (id) or all pending tasks")]
        Show {
            id: Option<u8>,
            // TODO: filter not implement
            #[arg(short, long, value_enum)]
            filter: Option<Filter>,
        },
        #[command(about = "Show all tasks")]
        ShowAll {

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
            project: Option<String>,

        },
        #[command(about = "Change the status of the task (id) to CANCELED")]
        Delete {
            id: u8,
        },
        #[command(about = "Delete the task (id) from the database (CAN NOT BE RECOVERED)")]
        Destroy {
            id: u8,
        },
        #[command(about = "Change the status of the task (id) to COMPLETED")]
        Done {
            id: u8,
        },
        #[command(about = "start the task (id)")]
        Start {
            id: u8,
        },
        #[command(about = "stop the task (id)")]
        Stop {
            id: u8,
        },
        #[command(about = "Delete all tasks from the database (CAN NOT BE RECORVERED)")]
        Clear {

        },
        #[command(about = "Show some blahblahblah from the author aka me")]
        Info {

        },
    }
}
