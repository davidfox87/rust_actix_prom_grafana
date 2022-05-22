//create custom data types
use chrono::{DateTime, Utc}; // https://docs.rs/chrono/latest/chrono/
use chrono::prelude::*;
use std::str;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub struct Task {
    pub time: DateTime<Utc>,
    pub text: String,
    pub priority: u8,
    pub first_name: String,
    pub last_name: String
}

impl Task {
    // Constructor
    pub fn new(time: DateTime<Utc>, 
                text: String, 
                priority: u8,
                first_name: String,
                last_name: String) -> Self {
        Self {time, text, priority, first_name, last_name}
    }

    /// Returns the value in seconds.
    pub fn priority(&self) -> u8 {
        self.priority
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = priority;
    }

    //no semicolon for functions that return something.
    // we told Rust that we will return something so 
    // it will return the thing that we don't end with a 
    //semicolon
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

// pub fn run() {

//     let dt = Utc.ymd(2022, 6, 18).and_hms(17, 0, 0); 
//     // we can just instantiate the class without the constructor
//     // let mut task = Task {
//     //     time: dt,
//     //     text: String::from("Get Married"),
//     //     priority: 2
//     // };
//     // task.priority = 1;

//     // println!("Task: {} {} {}", task.time, task.text, task.priority);

//     let mut task = Task::new(dt, String::from("Get Married"), 1, "David".to_string(), "Fox".to_string());
//     println!("Task: {} {} at {} with priority {}", task.full_name(), task.text, task.time, task.priority);
//     task.set_priority(2);
//     println!("Task: {} {} at {} with priority {}", task.full_name(), task.text, task.time, task.priority);

// }