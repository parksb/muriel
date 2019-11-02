#[derive(Debug)]
pub struct Book {
    pub author: String,
    pub title: String,
    pub publisher: String,
    pub published_at: String,
}

impl Book {
    pub fn new(
        author: String,
        title: String,
        publisher: String,
        published_at: String
    ) -> Self {
        Self {
            author,
            title,
            publisher,
            published_at,
        }
    }
}
