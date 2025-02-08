#[allow(dead_code)]
pub mod task {
    use tabled::Tabled;
    use std::borrow::Cow;
    use rusqlite::ToSql;
    use serde_derive::{Serialize, Deserialize};
    use serde_rusqlite::*;
    use chrono::{
        NaiveDate, Local, TimeDelta,
    };
    use clap::ValueEnum;

    const STORE_PATH: &str = "~/.taskoto/task";
    const DATE_FORMAT: &str = "%Y-%m-%d";

    type Date = Option<String>;

    /*
    * 0 - Urgent - Red
    * 1 - Started - Blink
    * 2 - Canceled - StrkeThrough
    * 3 - Completed - Dimmed
    * 4 - Failed & Expired
    * 5,6,7 - Reserved
    */
    pub type StateWord = u8;

    #[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)] 
    pub enum TaskStatus {
        Pending,
        Canceled,
        Completed,
        Expired,
        Failed,
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, ValueEnum)]
    pub enum Filter {
        Urgent,
        Failed,
        Done,
        Canceled,
        Expired,
        Started,
        NotStarted,
        Today,
        Tomorrow,
    }



    #[derive(Deserialize, Serialize)]
    pub struct Task {
        pub id: i32,
        pub name: String,
        pub status: TaskStatus,
        pub due: Date, 
        pub scheduled: Date, 
        pub start_time: Date, 
        pub end_time: Date, 
        pub project: Option<String>, 
        pub _is_started: bool,
    }

    /* Methods for structs */
    // Task

    impl Task {
        pub fn new() -> Self {
            Self {
                id: 1,
                name: "default_name".to_string(),
                status: TaskStatus::Pending,
                due: None,
                scheduled: None,
                start_time: None,
                end_time: None,
                project: None,
                _is_started: false,
            }
        }
        fn set_id(&mut self, id: i32) {
            self.id = id;
        }

        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }

        fn set_status(&mut self, status: i32) {
            match status {
                0 => self.status = TaskStatus::Pending,
                1 => self.status = TaskStatus::Canceled,
                2 => self.status = TaskStatus::Completed,
                3 => self.status = TaskStatus::Expired,
                4 => self.status = TaskStatus::Failed,
                _ => {},
            };
        }

        pub fn set_project(&mut self, project: Option<String>) {
            self.project = project;
        }

        pub fn start(&mut self) {
            if !self._is_started {
                self._is_started = true;
                let now = Local::now().date_naive().format(DATE_FORMAT).to_string();
                self.set_date(Some(now), 2);
            }
        }

        pub fn end(&mut self) {
            if self._is_started {
                self._is_started = false;
                let now = Local::now().date_naive().format(DATE_FORMAT).to_string();
                self.set_date(Some(now), 3);
            }
        }

        pub fn done(&mut self) {
            self.set_status(2);
            self._is_started = false;
            let now = Local::now().date_naive().format(DATE_FORMAT).to_string();
            self.set_date(Some(now), 3);
        }

        pub fn delete(&mut self) {
            self.set_status(1);
            self._is_started = false;
        }

        pub fn set_date(&mut self, date: Date, date_type: u8) {
            if let Some(p) = date {
                match NaiveDate::parse_from_str(&p, DATE_FORMAT) {
                    Ok(_) => {
                        match date_type {
                            0 => self.due = Some(p),
                            1 => self.scheduled = Some(p),
                            2 => self.start_time = Some(p),
                            3 => self.end_time = Some(p),
                            _ => {},
                        }
                    },
                    // TODO: Execption
                    Err(_) => {},

                }
            } else {
                match date_type {
                    0 => self.due = None,
                    1 => self.scheduled = None,
                    2 => {
                        self.start_time = None;
                        self._is_started = false;
                    },
                    3 => {
                        self.end_time = None;
                        self._is_started = match self.start_time {
                            Some(_) => true,
                            None => false,
                        }
                    },
                    _ => {},
                }
            }
        }

        pub fn verify(&mut self) {
            let now = Local::now().date_naive();
            match &self.status {
                TaskStatus::Pending => {
                    if let Some(date) = &self.due {
                        let due = NaiveDate::parse_from_str(date, DATE_FORMAT).unwrap();
                        match due.cmp(&now) {
                            std::cmp::Ordering::Less => self.set_status(4),
                            _ => {},
                        }
                    } else if let Some(date) = &self.scheduled {
                        let sch = NaiveDate::parse_from_str(date, DATE_FORMAT).unwrap();
                        match sch.cmp(&now) {
                            std::cmp::Ordering::Less => self.set_status(3),
                            _ => {},
                        }
                    }

                }
                _ => {},
            }
        }

        pub fn get_state_word(&self) -> StateWord {
            let mut state_word: StateWord = 0;
            if self.is_urgent() {
                state_word |= 1 << 0;
            }
            if self._is_started {
                state_word |= 1 << 1;
            }
            match self.status {
                TaskStatus::Canceled => state_word |= 1 << 2,
                TaskStatus::Completed => state_word |= 1 << 3,
                TaskStatus::Pending => {},
                _ => state_word |= 1 << 4,
            }
            state_word
        }

        fn is_urgent(&self) -> bool {
            if self.status != TaskStatus::Pending {
                return false;
            }
            let now = Local::now().date_naive();
            if let Some(due) = &self.due {
                let due = NaiveDate::parse_from_str(due, DATE_FORMAT).unwrap();
                due - now <= TimeDelta::days(3)
            } else {
                false
            }
        }

        pub fn filtered(&self, filter: &Filter) -> bool {
            match filter {
                Filter::Urgent => {
                    self.is_urgent()
                },
                Filter::Failed => {
                    self.status == TaskStatus::Failed
                },
                Filter::Done => {
                    self.status == TaskStatus::Completed
                },
                Filter::Canceled => {
                    self.status == TaskStatus::Canceled
                },
                Filter::Expired => {
                    self.status == TaskStatus::Expired
                },
                Filter::Started => {
                    self.status == TaskStatus::Pending && self._is_started
                },
                Filter::NotStarted => {
                    self.status == TaskStatus::Pending && !self._is_started
                },
                Filter::Today => {
                    if self.status != TaskStatus::Pending {
                        false
                    } else { 
                        let now = Local::now().date_naive();
                        if let Some(due) = &self.due {
                            let due = NaiveDate::parse_from_str(due, DATE_FORMAT).unwrap();
                            due - now == TimeDelta::days(0)
                        } else {
                            false
                        }
                    }
                },
                Filter::Tomorrow => {
                    if self.status != TaskStatus::Pending {
                        false
                    } else { 
                        let now = Local::now().date_naive();
                        if let Some(due) = &self.due {
                            let due = NaiveDate::parse_from_str(due, DATE_FORMAT).unwrap();
                            due - now == TimeDelta::days(1)
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
    // Date
    // impl Date {
    //     fn ret(&self) -> String {
    //         match &self.0 {
    //             Some(p) => p.to_string(),
    //             None => "".to_string(),
    //         }
    //     }
    // }
    fn get_date(date: &Option<String>) -> String {
        match date {
            Some(p) => p.to_string(),
            None => " - ".to_string(),
        }
    }


    /* Traits for structs */
    // Tosql
    impl ToSql for TaskStatus {
        fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
            let val = match self {
                Self::Pending => 0,
                Self::Canceled => 1,
                Self::Completed => 2,
                Self::Expired => 3,
                Self::Failed => 4,
            };
            let res = rusqlite::types::Value::Integer(val);
            Ok(rusqlite::types::ToSqlOutput::Owned(res))
        }

    }

    // Tabled
    impl Tabled for Task {
        const LENGTH: usize = 0;
        fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
            let id = Cow::from(self.id.to_string());
            let name = Cow::from(self.name.clone());
            let due = Cow::from(get_date(&self.due));
            let scheduled = Cow::from(get_date(&self.scheduled));
            let project = Cow::from(
                match &self.project {
                    Some(p) => p.clone(),
                    None => " - ".to_string(),
                }
            );
            // TODO: status can show in a shorter way
            let status = Cow::from(
                match &self.status {
                    TaskStatus::Pending => "Pending",
                    TaskStatus::Canceled => "Canceled",
                    TaskStatus::Completed => "Completed",
                    TaskStatus::Expired => "Expired",
                    TaskStatus::Failed => "Failed",
                }
            );
            vec![id, project, due, scheduled, name, status]
        }
        fn headers() -> Vec<std::borrow::Cow<'static, str>> {
            vec![
                Cow::from("ID"),
                Cow::from("Project"),
                Cow::from("D"),
                Cow::from("S"),
                Cow::from("Description"),
                Cow::from("Status"),
            ]
        }

    }

    pub fn sort_tasks(tasks: &mut Vec<Task>) {
        tasks.sort_unstable_by_key(|task|{
            task.status.clone()
        });
    }
}
