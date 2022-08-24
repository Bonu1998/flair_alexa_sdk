use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ControlMedia {
        command_type: String,
        component_id: String,
        command: String,
        value: Option<i32>     
}

impl ControlMedia{
    pub fn new(component_id:String, command:String, value:Option<i32>)-> ControlMedia{
        ControlMedia { command_type: "ControlMedia".to_string(), component_id, command, value }
    }
}