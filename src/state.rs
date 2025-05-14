
use std::collections::HashMap;


use std::sync::Mutex;
use uuid::Uuid;
use crate::models::Todo;

pub type TodoStore = Mutex<HashMap<Uuid, Todo>>;