use serde::{Deserialize, Serialize};

pub mod cmd;
pub mod tlm;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Database {
    pub components: Vec<Component>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub tlm: tlm::Database,
    pub cmd: cmd::Database,
}
