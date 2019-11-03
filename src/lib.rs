use std::fs;

#[derive(Debug)]
pub struct Book {
    pub id: i32,
    pub author: String,
    pub title: String,
    pub publisher: String,
    pub published_at: String,
    pub pages: i32,
    pub page_at: i32,
}

impl Book {
    pub fn new(
        author: String,
        title: String,
        publisher: String,
        published_at: String,
        pages: i32,
    ) -> Self {
        let data = fs::read_dir("./data").expect("Failed to read directory");
        let id = (data.count() + 1) as i32;

        Self {
            id,
            author,
            title,
            publisher,
            published_at,
            pages,
            page_at: 0,
        }
    }
}
