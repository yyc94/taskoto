#[allow(dead_code)]
pub mod project {
    use std::borrow::Cow;
    use chrono::{NaiveDate, Local, Datelike};
    use tabled::Tabled;
    use crate::*;


    #[derive(Deserialize, Serialize)]
    pub struct Project {
        pub id: i32,
        pub name: String,
        _progress: i32,
        deadline: Option<String>,
        description: Option<String>,
        is_done: bool,
    }

    impl Project {
        pub fn new() -> Self {
            Self {
                id: 1,
                name: String::from("Default Project"),
                _progress: 0,
                deadline: None,
                description: None,
                is_done: false,
            }
        }

        pub fn set_name(&mut self, name: String) {
            self.name = name; 
        }

        pub fn set_deadline(&mut self, date: Option<String>) {
            match date {
                Some(d) => {
                    let (flag, mut fmt) = get_date_format();
                    let mut input = d.clone();
                    if flag {
                        let year = Local::now().year().to_string();
                        input = year + "-" + &input;
                        fmt = String::from("%Y-") + &fmt;
                    }
                    match NaiveDate::parse_from_str(&input, &fmt) {
                        Ok(nd) => self.deadline = Some(nd.format(DATE_FORMAT).to_string()),
                        Err(_) => {}
                    }
                }, 
                None => self.deadline = None,
            }
        }

        pub fn set_description(&mut self, contents: String) {
            self.description = Some(contents);
        }

        fn project_done(&mut self) {
            self.is_done = true;
        }
    }
    impl Tabled for Project{
        const LENGTH: usize = 200;
        fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
            let id = Cow::from(self.id.to_string());
            let name = Cow::from(self.name.clone());
            let deadline = Cow::from(
                match &self.deadline {
                    Some(d) => d,
                    None => "-",
                });
            vec![id, name, deadline]
        }
        fn headers() -> Vec<std::borrow::Cow<'static, str>> {
            vec![
                Cow::from("ID"),
                Cow::from("Project"),
                Cow::from("D"),
            ]
        }
    }

}
