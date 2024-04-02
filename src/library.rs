use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::book::Book;

pub struct Library {
    books: HashMap<String, Book>,
}

impl Library {
    pub fn new() -> Library {
        Library {
            books: HashMap::new(),
        }
    }

    pub fn add_book(&mut self, title: &str, author: &str) -> Result<(), &'static str> {
        if self.books.contains_key(title) {
            Err("buku dengan judul tersebut sudah ada di dalam perpustakaan")
        } else {
            let book = Book::new(title, author);
            self.books.insert(title.to_string(), book);
            Ok(())
        }
    }

    pub fn find_book_by_title(&self, title: &str) -> Result<&Book, &'static str> {
        if let Some(book) = self.books.get(title) {
            Ok(book)
        } else {
            Err("buku tidak ditemukan di dalam perpustakaan")
        }
    }

    pub fn find_all_books(&self) -> Result<Vec<&Book>, &'static str> {
        if self.books.is_empty() {
            Err("buku tidak ditemukan di dalam perpustakaan")
        } else {
            Ok(self.books.values().collect())
        }
    }

    pub fn edit_book(
        &mut self,
        title: &str,
        new_title: &str,
        new_author: &str,
    ) -> Result<(), &'static str> {
        if let Some(book) = self.books.get_mut(title) {
            book.title = new_title.to_string();
            book.author = new_author.to_string();
            if let Some(_) = self.books.remove(title) {
                self.books
                    .insert(new_title.to_string(), Book::new(new_title, new_author));
                Ok(())
            } else {
                Err("buku gagal diubah")
            }
        } else {
            Err("buku tidak ditemukan di dalam perpustakaan")
        }
    }

    pub fn remove_book(&mut self, title: &str) -> Result<(), &'static str> {
        if self.books.remove(title).is_some() {
            Ok(())
        } else {
            Err("buku tidak ditemukan di dalam perpustakaan")
        }
    }
}

lazy_static! {
    pub static ref LIBRARY: Arc<Mutex<Library>> = Arc::new(Mutex::new(Library::new()));
}
