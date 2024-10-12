// #[derive(Debug)]
// enum Media {
//     Book { title: String, author: String },
//     Movie { title: String, director: String },
//     Audiobook { title: String },
//     Podcast(u32),
//     Placeholder,
// }

// impl Media {
//     fn description(&self) -> String {
//         // if let Media::Book { title, author } = self {
//         //     format!("Book: {} {}", title, author)
//         // } else if let Media::Movie { title, director } = self {
//         //     format!("Movie: {} {}", title, director)
//         // } else if let Media::Audiobook { title } = self {
//         //     format!("Audiobook: {}", title)
//         // } else {
//         //     String::from("Media description")
//         // }

//         match self {
//             Media::Book { title, author } => {
//                 format!("Book: {} {}", title, author)
//             }
//             Media::Movie { title, director } => {
//                 format!("Movie: {} {}", title, director)
//             }
//             Media::Audiobook { title } => {
//                 format!("Audiobook: {}", title)
//             }
//             Media::Podcast(id) => {
//                 format!("Podcast: {}", id)
//             }
//             Media::Placeholder => {
//                 format!("Placeholder")
//             }
//         }
//     }
// }

// #[derive(Debug)]
// struct Catalog {
//     items: Vec<Media>,
// }

// impl Catalog {
//     fn new() -> Self {
//         Catalog { items: vec![] }
//     }

//     fn add(&mut self, media: Media) {
//         self.items.push(media);
//     }

//     fn get_by_index(&self, index: usize) -> Option<&Media> {
//         if self.items.len() > index {
//             // Good! We have somethign to return
//             Some(&self.items[index])
//         } else {
//             // Bad! We don't have anything to return!!!
//             None
//         }
//     }
// }

// fn print_media(media: Media) {
//     println!("{:#?}", media);
// }

// fn main() {
//     let audiobook = Media::Audiobook {
//         title: String::from("An Audiobook"),
//     };
//     let good_movie = Media::Movie {
//         title: String::from("Good Movie"),
//         director: String::from("Good Director"),
//     };
//     let bad_book = Media::Book {
//         title: String::from("Bad Book"),
//         author: String::from("Bad Author"),
//     };
//     let podcast = Media::Podcast(10);
//     let placeholder = Media::Placeholder;

//     // println!("{}", audiobook.description());
//     // println!("{}", good_movie.description());
//     // println!("{}", bad_book.description());

//     let mut catalog = Catalog::new();

//     catalog.add(audiobook);
//     catalog.add(good_movie);
//     catalog.add(bad_book);
//     catalog.add(podcast);
//     catalog.add(placeholder);

//     let item = catalog.get_by_index(40);

//     println!("{:#?}", item);

//     // match catalog.get_by_index(9999) {
//     //     Some(value) => {
//     //         println!("Item: {:#?}", value);
//     //     }
//     //     None => {
//     //         println!("No value here!");
//     //     }
//     // }

//     // if let Some(value) = catalog.get_by_index(0) {
//     //     println!("Item in pattern match: {:#?}", value)
//     // } else {
//     //     println!("No value!!!!!!!!!!");
//     // }
// }




// TODO:
// 1) Safely access the first account in the 'accounts' vector using the 
//    .first_mut() method. 
// 2) '.first_mut()' returns an Option whose Some variant is a mutable ref to 
//     an Account. Use a 'match' statement to figure out if
//     you have a Some or a None
// 3) In the Some case, set the balance of the account to 30, then print the account
// 4) In the None case, print the message "No account found"
// Hint: You might have to add in the 'mut' keyword somewhere...

use std::vec;

#[derive(Debug)]
struct Account {
    balance: i32
}

fn main() {
    let mut accounts: Vec<Account> = vec![
        Account { balance: 0 },
        Account { balance: 10 }
    ];

    // println!("{:#?}", accounts.first_mut());

    match accounts.first_mut(){
        Some(value)=>{
            value.balance = 30;
            println!("{:#?}", value)
        }
        None=>{
            println!("No Account Found");
        }
    }
    
    // Add code here:
}