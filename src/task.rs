#[allow(dead_code)]
pub mod task {
    use tabled::Tabled;
    use std::borrow::Cow;
    use rusqlite::ToSql;
    use serde_derive::{Serialize, Deserialize};
    use serde_rusqlite::*;
    use chrono::{
        NaiveDate, Local, TimeDelta, Days, Datelike, Weekday,
    };
    use clap::ValueEnum;

    use crate::*;
    use crate::project::project::Project;

    const STORE_PATH: &str = "~/.taskoto/task";



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

    enum DateType {
        Tomorrow,
        Today,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
        RegularDate,
    }


    #[derive(Deserialize, Serialize)]
    pub struct Task {
        pub id: i32,
        name: String,
        pub status: TaskStatus,
        due: Date, 
        scheduled: Date, 
        start_time: Date, 
        end_time: Date, 
        pub project_id: Option<i32>,
        project: Option<String>, 
        _is_started: bool,
        _urgent: f32,
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
                project_id: None,
                project: None,
                _is_started: false,
                _urgent: 1f32,
            }
        }

        // fn set_id(&mut self, id: i32) {
        //     self.id = id;
        // }

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

        pub fn set_project(&mut self, project: Option<Project>) {
            if let Some(pro) = project {
                self.project_id = Some(pro.id);
                self.project = Some(pro.name);
            } else {
                self.project_id = None;
                self.project= None;
            }
        }

        pub fn start(&mut self) {
            if !self._is_started {
                self._is_started = true;
                let now = Local::now().date_naive().format(DATE_FORMAT).to_string();
                self.set_date(Some(now), 2);
            }
        }

        pub fn stop(&mut self) {
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

        pub fn set_date(&mut self, date: Date, date_type: u8){
            let enum_to_u8 = |weekday: Weekday| -> u8 {
                match weekday {
                    Weekday::Mon => 1,
                    Weekday::Tue => 2,
                    Weekday::Wed => 3,
                    Weekday::Thu => 4,
                    Weekday::Fri => 5,
                    Weekday::Sat => 6,
                    Weekday::Sun => 7,
                }
            };
            let calc_duration = |a: u8, b:u8| -> u8 {
                if a < b {
                    b - a
                } else {
                    7 - a + b
                } 
            };
            let dds = if let Some(p) = date {
                let now = Local::now().date_naive();
                let weekday = enum_to_u8(now.weekday());
                match p.to_lowercase().as_str() {
                    "mon" | "monday" => {
                        let dura = calc_duration(weekday, 1);
                        Some(now
                            .checked_add_days(Days::new(dura as u64))
                            .unwrap()
                            .format(DATE_FORMAT).to_string())
                    },
                    "tue" | "tuesday" => {
                        let dura = calc_duration(weekday, 2);
                        Some(now
                            .checked_add_days(Days::new(dura as u64))
                            .unwrap()
                            .format(DATE_FORMAT).to_string())
                    },
                    "wed" | "wednsday" => {
                        let dura = calc_duration(weekday, 3);
                        Some(now
                            .checked_add_days(Days::new(dura as u64))
                            .unwrap()
                            .format(DATE_FORMAT).to_string())
                    },
                    "thu" | "thursday" => {
                        let dura = calc_duration(weekday, 4);
                        Some(now
                            .checked_add_days(Days::new(dura as u64))
                            .unwrap()
                            .format(DATE_FORMAT).to_string())
                    },
                    "fri" | "friday" => {
                        let dura = calc_duration(weekday, 5);
                        Some(now
                            .checked_add_days(Days::new(dura as u64))
                            .unwrap()
                            .format(DATE_FORMAT).to_string())
                    },
                    "sat" | "saturday" => {
                        let dura = calc_duration(weekday, 6);
                        Some(now
                            .checked_add_days(Days::new(dura as u64))
                            .unwrap()
                            .format(DATE_FORMAT).to_string())
                    },
                    "sun" | "sunday" => {
                        let dura = calc_duration(weekday, 7);
                        Some(now
                            .checked_add_days(Days::new(dura as u64))
                            .unwrap()
                            .format(DATE_FORMAT).to_string())
                    },
                    "now" | "today" => {
                        Some(now
                            .format(DATE_FORMAT).to_string())
                    },
                    "later" | "tomorrow" => {
                        Some(now
                            .checked_add_days(Days::new(1))
                            .unwrap()
                            .format(DATE_FORMAT).to_string())
                    },
                    _ => {
                        let mut input = p.clone();
                        let (flag, mut fmt)= get_date_format();
                        if flag {
                            let year = Local::now().year().to_string();
                            input = year + "-" + &input;
                            fmt = String::from("%Y-") + &fmt;
                        }
                        match NaiveDate::parse_from_str(&input, &fmt) {
                            Ok(date) => Some(date.format(DATE_FORMAT).to_string()),
                            Err(_) => None,
                        }
                    },
                }
            } else {
                None
            };
            match date_type {
                0 => self.due = dds,
                1 => self.scheduled = dds,
                2 => self.start_time = dds,
                3 => self.end_time = dds,
                _ => {}, 
            }
            /*
            if let Some(p) = date {
            match date_string_to_enum(&p) {
            WeekDay::Tomorrow => {
            let mut now = Local::now().date_naive();
            now.checked_add_days(Days::new(1)).unwrap();
            }
            }
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
            */
        }
        fn calc_urgent(&mut self) {
            match self.status {
                TaskStatus::Pending => {
                    self._urgent = 1f32;
                    if let Some(due) = &self.due {
                        let deadline = NaiveDate::parse_from_str(due, DATE_FORMAT).unwrap();
                        let now = Local::now().date_naive();
                        let period = (deadline - now).num_days() as f32;
                        self._urgent += 1f32 / (period + 1f32);
                    }
                    if self._is_started {
                        self._urgent += 1.5;
                    }

                },
                TaskStatus::Completed => self._urgent = 0.5,
                _ => self._urgent = 0f32,
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
            self.calc_urgent();
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
            if self._is_started {
                self._urgent > 2.8
            } else {
                self._urgent > 1.3
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
            Some(p) => {
                let d = NaiveDate::parse_from_str(p, DATE_FORMAT).unwrap();
                let (_, display_format) = get_date_format();
                d.format(&display_format).to_string()
            },
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
        const LENGTH: usize = 200;
        fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
            let id = Cow::from(self.id.to_string());
            let name = Cow::from(self.name.clone());
            let due = Cow::from(get_date(&self.due));
            let scheduled = Cow::from(get_date(&self.scheduled));
            let project = Cow::from(
                if let Some(id) = self.project_id {
                    format!("[{}]{}", id, self.project.clone().unwrap())
                } else {
                    "-".to_string()
                }
            );
            // TODO: status can show in a shorter way
            let status = Cow::from(
                match &self.status {
                    TaskStatus::Pending => "P",
                    TaskStatus::Canceled => "C",
                    TaskStatus::Completed => "D",
                    TaskStatus::Expired => "E",
                    TaskStatus::Failed => "F",
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
        tasks.sort_unstable_by(|a, b| 
            b._urgent.partial_cmp(&a._urgent).unwrap())
    }

}
