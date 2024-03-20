use crate::book::Book;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref LIBRARY_INSTANCE: Arc<Mutex<Library>> = Arc::new(Mutex::new(Library::new()));
}

pub struct Library {
    pub books: HashMap<String, Book>,
}

impl Library {
    pub fn new() -> Self {
        Library {
            books: HashMap::new(),
        }
    }

    pub fn get_books(&self) -> Option<Vec<&Book>> {
        let books = self.books.values().collect::<Vec<&Book>>();
        if books.is_empty() {
            None
        } else {
            Some(books)
        }
    }

    pub fn get_book_by_title(&self, title: &str) -> Option<&Book> {
        self.books.get(title)
    }

    pub fn add_book(&mut self, title: &str, author: &str) -> Option<&Book> {
        if self.books.contains_key(title) {
            None
        } else {
            let book = Book {
                title: title.to_string(),
                author: author.to_string(),
            };
            self.books.insert(title.to_string(), book);
            self.books.get(title)
        }
    }

    pub fn edit_book(&mut self, title: &str, new_title: &str, new_author: &str) -> Option<Book> {
        if let Some(mut book) = self.books.remove(title) {
            let old_title = book.title.clone();
            book.title = new_title.to_string();
            book.author = new_author.to_string();
            self.books.insert(book.title.clone(), book);
            Some(Book {
                title: old_title,
                author: new_author.to_string(),
            })
        } else {
            None
        }
    }

    pub fn remove_book(&mut self, title: &str) -> Option<Book> {
        self.books.remove(title)
    }
}
