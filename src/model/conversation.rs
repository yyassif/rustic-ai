use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
  pub id: Uuid,
  pub name: String,
  pub messages: Vec<Message>
}

impl Conversation {
  pub fn new(name: String) -> Conversation {
    Conversation {
      name: name,
      id: Uuid::new_v4(),
      messages: Vec::new(),
    }
  } 
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
  pub id: Uuid,
  pub role: String,
  pub content: String,
  pub timestamp: String
}