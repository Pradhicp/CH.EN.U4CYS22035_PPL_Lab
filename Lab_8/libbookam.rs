#[derive(Clone, Debug)]  
struct Book {
    title: String,
    author: String,
    isbn: String,
    is_issued: bool,
}

impl Book {
    fn new(title: &str, author: &str, isbn: &str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            isbn: isbn.to_string(),
            is_issued: false,
        }
    }

    fn issue_book(self) -> Book {
        if self.is_issued {
            println!("The book '{}' is already issued.", self.title);
            self  
        } else {
            let mut issued_book = self;
            issued_book.is_issued = true;
            println!("The book '{}' has been issued.", issued_book.title);
            issued_book  
        }
    }

    fn return_book(&mut self) {
        if self.is_issued {
            self.is_issued = false;
            println!("The book '{}' has been returned.", self.title);
        } else {
            println!("The book '{}' was not issued.", self.title);
        }
    }

    fn display_details(&self) {
        println!("Title: {}, Author: {}, ISBN: {}, Issued: {}", 
                 self.title, self.author, self.isbn, self.is_issued);
    }
}

fn main() {
    let book1 = Book::new("Rust Programming", "Steve Klabnik", "123-456-789");

    book1.display_details();  

    let backup_book = book1.clone();
    println!("Backup created for book: {:?}", backup_book);

    let mut issued_book = book1.issue_book();


    issued_book.display_details();  

    issued_book.return_book();

    issued_book.display_details();  

    println!("Library backup book remains: {:?}", backup_book); 
}
