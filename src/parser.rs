pub mod parser {
    use clap::{Parser, Subcommand};
    use crate::task::task::Filter;

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
}
