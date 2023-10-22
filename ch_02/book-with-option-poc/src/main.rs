fn main() {
    struct Book {
        title: String,
        isbn: Option<String>,
    }

    let book = Book {
        title: "The Rust Programming Language".to_string(),
        isbn: Some(String::from("978-1-59327-828-1")),
    };

    match book.isbn {
        Some(isbn) => println!(
            "The ISBN of the book: {} is: {}",
            book.title,
            isbn
        ),
        None => println!("We don't know the ISBN of {}", book.title),
    }
}
