pub struct Book {
    pub title: String,
    pub author: String,
}

impl Book {
    pub fn new(title: &str, author: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
        }
    }
}
