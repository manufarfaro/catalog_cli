#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        if let Media::Audiobook { title } = self {
            format!("Audiobook: {}", title)
        } else if let Media::Movie { title, director } = self {
            format!("Movie: {}, {}", title, director)
        } else if let Media::Book { title, author } = self {
            format!("Book: {}, {}", title, author)
        } else {
            String::from("Type not supported")
        }
    }
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An audiobook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());
}
