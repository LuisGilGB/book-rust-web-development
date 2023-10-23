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

fn main() {
    let question = Question::new(
        QuestionId("1".to_string()),
        "What is the meaning of life?".to_string(),
        "42".to_string(),
        Some(vec!["life".to_string(), "meaning".to_string()]),
    );
    println!("Question: {:?}", question);
}
