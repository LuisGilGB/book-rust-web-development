use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct QuestionId(pub i32);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuestionDraft {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl fmt::Display for Question {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Question: {}, {}, {}, {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

impl fmt::Display for QuestionId {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_id_from_str() {
        let id = 1234;
        let question_id = QuestionId::from_str(id).unwrap();
        assert_eq!(question_id, QuestionId(id));
    }

    #[test]
    fn test_question_display() {
        let id = 1234;
        let title = "title".to_string();
        let content = "content".to_string();
        let tags = Some(vec!["tag1".to_string(), "tag2".to_string()]);
        let question = Question::new(
            QuestionId(id),
            title.clone(),
            content.clone(),
            tags.clone(),
        );
        let question_str = format!("Question: {}, {}, {}, {:?}", id, title, content, tags);
        assert_eq!(question.to_string(), question_str);
    }
}
