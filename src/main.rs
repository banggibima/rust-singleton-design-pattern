mod book;
mod library;

use library::Library;

fn main() {
    let mut my_library = Library::new();

    if let Some(added_book) = my_library.add_book(
        "Atomic Habits Perubahan Kecil yang Memberikan Hasil Luar Biasa",
        "James Clear",
    ) {
        println!("buku berhasil ditambahkan: {:?}", added_book);
    } else {
        println!("buku gagal ditambahkan, buku dengan judul yang sama sudah ada");
    }

    if let Some(added_book) =
        my_library.add_book("How to Win Friends and Influence People", "Dale Carnegie")
    {
        println!("buku berhasil ditambahkan: {:?}", added_book);
    } else {
        println!("buku gagal ditambahkan, buku dengan judul yang sama sudah ada");
    }

    if let Some(added_book) =
        my_library.add_book("Berani Tidak Disukai", "Ichiro Kishimi, Fumitake Koga")
    {
        println!("buku berhasil ditambahkan: {:?}", added_book);
    } else {
        println!("buku gagal ditambahkan, buku dengan judul yang sama sudah ada");
    }

    if let Some(edited_book) = my_library.edit_book(
        "How to Win Friends and Influence People",
        "Nunchi Seni Membaca Pikiran dan Perasaan Orang Lain",
        "Euny Hong",
    ) {
        println!("buku berhasil diubah: {:?}", edited_book);
    } else {
        println!("buku tidak ditemukan untuk diubah");
    }

    if let Some(removed_book) = my_library.remove_book("Berani Tidak Disukai") {
        println!("buku berhasil dihapus: {:?}", removed_book);
    } else {
        println!("buku tidak ditemukan untuk dihapus");
    }

    if let Some(all_books) = my_library.get_books() {
        if all_books.is_empty() {
            println!("buku kosong");
        } else {
            println!("list buku: ");
            for book in all_books {
                println!("- {:?}", book);
            }
        }
    } else {
        println!("buku kosong");
    }

    if let Some(book) =
        my_library.get_book_by_title("Nunchi Seni Membaca Pikiran dan Perasaan Orang Lain")
    {
        println!("buku berdasarkan nama judul {}: ", book.title);
        println!("- {:?}", book);
    } else {
        println!("buku tidak ditemukan");
    }
}
