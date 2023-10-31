use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::answer::{Answer, AnswerId};
use crate::domain::question::{Question, QuestionId};

#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(HashMap::new())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn init() -> Self {
        let file = include_str!("../../questions.json");
        let parsed_hash_map: HashMap<QuestionId, Question> = serde_json::from_str(file).expect("Can't parse questions.json file");

        Store {
            questions: Arc::new(RwLock::new(parsed_hash_map)),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
