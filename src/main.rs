mod content;

use content::{catalog::Catalog, media::Media};

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
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(1);
    let placeholder = Media::Placeholder;

    println!("{:#?}", item.unwrap_or(&placeholder));
}
