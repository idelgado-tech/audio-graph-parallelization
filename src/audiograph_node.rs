//! This module implements a node for the audio graph

use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct AGNode {
    pub id: String, // `AGNode`'s ID

    // Pure Data related informations
    pub object_name: String, // Pure Data object's name
    pub xpos: i64,           // Pure Data node's X position
    pub ypos: i64,           // Pure Data node's Y position
    pub args: Vec<String>,   // Pure Data object's list of arguments
    pub nb_inlets: i64,
    pub nb_outlets: i64,
    pub text: Option<String>,
    pub wcet: Option<f64>,

    // Scheduling related informations
    pub estimated_computation_cost: Option<f64>, // In milliseconds
}

impl AGNode {
    /// Creates a new empty `AGNode`
    pub fn new() -> AGNode {
        AGNode {
            id: String::default(),

            object_name: String::default(),
            xpos: -1,
            ypos: -1,
            args: Vec::new(),
            nb_inlets: -1,
            nb_outlets: -1,
            text: None,
            wcet: None,

            estimated_computation_cost: None,
        }
    }

    /// Sets the `AGNode`'s position
    pub fn set_pos(&mut self, x: i64, y: i64) {
        self.xpos = x;
        self.ypos = y;
    }

    /// Adds an argument to the current list
    pub fn add_arg(&mut self, arg: String) {
        self.args.push(arg);
    }
}

impl fmt::Display for AGNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.object_name)
    }
}
