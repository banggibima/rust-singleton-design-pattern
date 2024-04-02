mod book;
mod library;

use library::LIBRARY;
use std::io::{self, Write};

fn main() {
    loop {
        print_option();
        print!("pilih opsi: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "1" => add_book(),
            "2" => find_book_by_title(),
            "3" => find_all_books(),
            "4" => edit_book(),
            "5" => remove_book(),
            "6" => break,
            _ => println!("input tidak valid"),
        }
    }
}

fn print_option() {
    println!("1. tambah buku");
    println!("2. cari buku berdasarkan judul");
    println!("3. cari semua buku");
    println!("4. ubah buku berdasarkan judul");
    println!("5. hapus buku berdasarkan judul");
    println!("6. keluar");
    println!();
}

fn add_book() {
    print!("judul: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();

    print!("penulis: ");
    io::stdout().flush().unwrap();
    let mut author = String::new();
    io::stdin().read_line(&mut author).unwrap();
    let author = author.trim();

    match LIBRARY.lock().unwrap().add_book(title, author) {
        Ok(_) => println!("buku berhasil ditambahkan"),
        Err(e) => println!("{}", e),
    }
    println!();
}

fn find_book_by_title() {
    print!("judul: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();

    match LIBRARY.lock().unwrap().find_book_by_title(title) {
        Ok(book) => println!("judul: {}, penulis: {}", book.title, book.author),
        Err(e) => println!("{}", e),
    }
    println!();
}

fn find_all_books() {
    match LIBRARY.lock().unwrap().find_all_books() {
        Ok(books) => {
            for book in books {
                println!("judul: {}, penulis: {}", book.title, book.author);
            }
        }
        Err(e) => println!("{}", e),
    }
    println!();
}

fn edit_book() {
    print!("judul: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();

    print!("judul baru: ");
    io::stdout().flush().unwrap();
    let mut new_title = String::new();
    io::stdin().read_line(&mut new_title).unwrap();
    let new_title = new_title.trim();

    print!("penulis baru: ");
    io::stdout().flush().unwrap();
    let mut new_author = String::new();
    io::stdin().read_line(&mut new_author).unwrap();
    let new_author = new_author.trim();

    match LIBRARY
        .lock()
        .unwrap()
        .edit_book(title, new_title, new_author)
    {
        Ok(_) => println!("buku berhasil diubah"),
        Err(e) => println!("{}", e),
    }
    println!();
}

fn remove_book() {
    print!("buku: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();

    match LIBRARY.lock().unwrap().remove_book(title) {
        Ok(_) => println!("buku berhasil dihapus"),
        Err(e) => println!("{}", e),
    }
    println!();
}
