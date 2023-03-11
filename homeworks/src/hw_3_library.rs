
use std::fmt::Display;
use guid_create::GUID;

struct Library
{
    books:Vec<Book>,
    location_city:String
}

struct  Book{
    id:GUID,
    name:String,
    author:String,
    book_type:BookType,
    page:u16,
    year:u16,
    publisher:String


}
#[derive(Debug)]
enum BookType
{
    WorldClassics,
    Novel,
    Story,
    Poem,
    ScienceFiction,
    SelfDevelopment
}

impl Book {
    fn new(name:String,author:String,book_type:BookType,page:u16,year:u16,publisher:String) -> Book {
        return Book{
            id:GUID::rand(),
            name:String::from(name),
            author:String::from(author),
            book_type:book_type,
            page:page,
            year:year,
            publisher:String::from(publisher)
        };
    }
}
impl Display for Book
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f,"Id : {} | Name: {} | Author: {} | Type: {:?} | Page: {} | Year: {} | Publisher: {}",
        self.id,self.name,self.author,self.book_type,self.page,self.year,self.publisher);
    }
}
impl  Library {
    fn new(city:String) ->Library{
        return Library { books: Vec::new(), location_city: city };
    }

    fn add_book(&mut self,book:Book)
    {
        self.books.push(book);
    }
    fn len(&self) ->usize
    {
        return self.books.len();
    }
    fn is_empty(&self) ->bool
    {
        return self.books.is_empty();
    }
    fn print_books(&self)
    {
        for book in &self.books
        {
            println!("{}",book);
        }
    }
    fn oldest_book(&self) ->Option<&Book>
    {
        if self.books.len()==0
        {
            return  None;
        }
        let mut result = &self.books[0];
        for item in &self.books
        {
            if item.year<result.year
            {
                result = item;
            }
        }
        return Some(result);
    }

}

fn main()
{
    let mut library = Library::new("Istanbul".to_string());
    println!("Is our library empty: {}", &library.is_empty());

    match library.oldest_book() {
        Some(book) => println!("The oldest book : {}",book),
        None => println!("Our library is empty"),
    }
    let book1 = Book::new("Martin Eden".to_string(), "Jack London".to_string(), BookType::WorldClassics, 420, 1860, "Kumsaati".to_string());
    library.books.push(book1);

    let book2 = Book::new("The Secret".to_string(), "Rhonda Byrne".to_string(), BookType::SelfDevelopment, 200, 2001, "Ekim".to_string());
    library.add_book(book2);

    library.print_books();
    println!("Our library has {} books", library.len());

    match library.oldest_book() {
        Some(book) => println!("The oldest book : {}",book),
        None => println!("Is our library empty"),
    }

}