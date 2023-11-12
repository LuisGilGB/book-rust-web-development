use serde::{Deserialize, Serialize};

use crate::domain::question::QuestionId;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct AnswerId(pub i32);
