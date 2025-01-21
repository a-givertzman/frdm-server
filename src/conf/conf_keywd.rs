#![allow(non_snake_case)]

use std::str::FromStr;
use log::{trace, warn};
use regex::RegexBuilder;
use serde::Deserialize;

///
/// 
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum ConfKind {
    Task,
    Service,
    Queue,
    Link,
    Unknown,
}
//
// 
impl FromStr for ConfKind {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "task" => Ok(ConfKind::Task),
            "service" => Ok(ConfKind::Service),
            "queue" => Ok(ConfKind::Queue),
            "link" => Ok(ConfKind::Link),
            _ => Err(format!("ConfKind.fron_str | Unknown keyword: '{}'", input))
        }
    }
}
//
// 
impl ToString for ConfKind {
    fn to_string(&self) -> String {
        match self {
            ConfKind::Task => "task",
            ConfKind::Service => "service",
            ConfKind::Queue => "queue",
            ConfKind::Link => "link",
            ConfKind::Unknown => "unknown",
        }.to_string()
    }
}
///
/// 
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct ConfKeywdValue {
    pub prefix: String,
    pub kind: ConfKind,
    pub name: String,
    pub sufix: String,
}

///
/// keyword konsists of 3 fields:
/// ```
/// | prefix |  kind  |  name               |
/// |        |        |                     |
/// |--------|--------|---------------------|
/// | opt    | requir |  requir             |
/// |--------|--------|---------------------|
/// |        | task   | Task1               |
/// |        | service| ApiClient           |
/// | in     | queue  | in-queue            |
/// | out    | queue  | out-queue           |
/// ````
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum ConfKeywd {
    Task(ConfKeywdValue),
    Service(ConfKeywdValue),
    Queue(ConfKeywdValue),
    Link(ConfKeywdValue),
}
//
// 
impl ConfKeywd {
    pub fn prefix(&self) -> String {
        match self {
            ConfKeywd::Task(v) => v.prefix.clone(),
            ConfKeywd::Service(v) => v.prefix.clone(),
            ConfKeywd::Queue(v) => v.prefix.clone(),
            ConfKeywd::Link(v) => v.prefix.clone(),
        }
    }
    pub fn kind(&self) -> ConfKind {
        match self {
            ConfKeywd::Task(v) => v.kind.clone(),
            ConfKeywd::Service(v) => v.kind.clone(),
            ConfKeywd::Queue(v) => v.kind.clone(),
            ConfKeywd::Link(v) => v.kind.clone(),
        }
    }
    pub fn name(&self) -> String {
        match self {
            ConfKeywd::Task(v) => v.name.clone(),
            ConfKeywd::Service(v) => v.name.clone(),
            ConfKeywd::Queue(v) => v.name.clone(),
            ConfKeywd::Link(v) => v.name.clone(),
        }
    }
    pub fn sufix(&self) -> String {
        match self {
            ConfKeywd::Task(v) => v.sufix.clone(),
            ConfKeywd::Service(v) => v.sufix.clone(),
            ConfKeywd::Queue(v) => v.sufix.clone(),
            ConfKeywd::Link(v) => v.sufix.clone(),
        }
    }
}
//
// 
impl FromStr for ConfKeywd {
    type Err = String;
    fn from_str(input: &str) -> Result<ConfKeywd, String> {
        trace!("FnConfKeywd.from_str | input: {}", input);
        // let re = r#"(?:(?:(\w+)|))(?:(?:\s|)(task|service|queue|link){1}(?:$|(?:[ \t]['"]*(\S+)['"]*)))"#;
        let re = r#"(?:(?:(\w+)[ \t])?(task|service|queue|link){1}(?:$|(?:[ \t](\S+)(?:[ \t](\S+))?)))"#;
        let re = RegexBuilder::new(re).multi_line(false).build().unwrap();
        let groupPrefix = 1;
        let groupKind = 2;
        let groupName = 3;
        let groupSufix = 4;
        match re.captures(input) {
            Some(caps) => {
                let prefix = match &caps.get(groupPrefix) {
                    Some(first) => String::from(first.as_str()),
                    None => String::new(),
                };
                let kind = match &caps.get(groupKind) {
                    Some(kind) => {
                        match ConfKind::from_str(&kind.as_str().to_lowercase()) {
                            Ok(kinde) => kinde,
                            Err(_err) => {
                                warn!("ConfKeywd.from_str | Error parsing kind of keyword '{}'", &input);
                                ConfKind::Unknown
                            }
                        }
                    }
                    None => ConfKind::Unknown,
                };
                let name = match &caps.get(groupName) {
                    Some(arg) => {
                        Ok(arg.as_str().to_string())
                    }
                    None => {
                        if input.is_empty() {                            
                            Err(format!("Error reading data of keyword '{}'", &input))
                        } else {
                            Ok(String::new())
                        }
                    }
                };
                let sufix = match &caps.get(groupSufix) {
                    Some(first) => String::from(first.as_str()),
                    None => String::new(),
                };
                match &name {
                    Ok(name) => {
                        match &caps.get(groupKind) {
                            Some(keyword) => {
                                match keyword.as_str() {
                                    "task"      => Ok( ConfKeywd::Task( ConfKeywdValue { prefix, kind, name: name.to_string(), sufix } )),
                                    "service"   => Ok( ConfKeywd::Service( ConfKeywdValue { prefix, kind, name: name.to_string(), sufix } )),
                                    "queue"     => Ok( ConfKeywd::Queue( ConfKeywdValue { prefix, kind, name: name.to_string(), sufix } )),
                                    "link"      => Ok( ConfKeywd::Link( ConfKeywdValue { prefix, kind, name: name.to_string(), sufix } )),
                                    _           => Err(format!("Unknown keyword '{:?}'", &keyword)),
                                }
                            }
                            None => {
                                Err(format!("Unknown keyword '{}'", &input))
                            }
                        }
                    }
                    Err(err) => Err(err.to_string()),
                }
            }
            None => {
                Err(format!("Prefix Kinde Name - not found in keyword '{}'", &input))
            }
        }
    }
}