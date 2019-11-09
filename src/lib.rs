use std::fs;
use serde::{Serialize, Deserialize};

pub enum BookField {
    Author,
    Title,
    Publisher,
    PublishedAt,
    Pages,
    PageAt,
}

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn set_author(&mut self, author: String) {
        self.author = author;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_publisher(&mut self, publisher: String) {
        self.publisher = publisher;
    }

    pub fn set_published_at(&mut self, published_at: String) {
        self.published_at = published_at;
    }

    pub fn set_pages(&mut self, pages: i32) {
        self.pages = pages;
    }

    pub fn set_page_at(&mut self, page_at: i32) {
        self.page_at = page_at;
    }
}
