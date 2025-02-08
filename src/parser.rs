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
        Init {},
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
        Show {
            id: Option<u8>,
            // TODO: filter not implement
            #[arg(short, long, value_enum)]
            filter: Option<Filter>,
        },
        ShowAll {

        },
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
        Delete {
            id: u8,
        },
        Destroy {
            id: u8,
        },
        Done {
            id: u8,
        },
        Start {
            id: u8,
        },
        End {
            id: u8,
        },
        Clear {

        },
    }
}
