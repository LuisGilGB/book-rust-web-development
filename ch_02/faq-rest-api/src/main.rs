use std::fmt;
use std::str::FromStr;

use warp::Filter;

#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
struct QuestionId(String);

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

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No id provided",
            )),
        }
    }
}

#[tokio::main]
async fn main() {
    let question = Question::new(
        QuestionId::from_str("1").expect("Unable to parse id"),
        "What is the meaning of life?".to_string(),
        "42".to_string(),
        Some(vec!["life".to_string(), "meaning".to_string()]),
    );

    let get_question = warp::get()
        .map(move || format!("Question: {}", question.title));

    warp::serve(get_question)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
