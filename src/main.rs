use rusqlite::{params, Connection};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug)]
struct Card {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn read_file_as_byte_vector(file_path: &str) -> Vec<u8> {
    let file_to_read = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file_to_read);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).unwrap();

    buffer
}

fn main() {
    let connection = Connection::open_in_memory().unwrap();
    let picture_1 = read_file_as_byte_vector("assets/1.jpg");

    let first_card = Card {
        id: 1,
        name: "Two pandas".to_string(),
        data: Some(picture_1),
    };

    connection
        .execute(
            "CREATE TABLE card (id PRIMARY KEY, name TEXT, file BLOB);",
            [],
        )
        .unwrap();

    connection
        .execute(
            "INSERT INTO card VALUES (?1, ?2, ?3)",
            params![first_card.id, first_card.name, first_card.data],
        )
        .unwrap();

    let mut statement = connection.prepare("SELECT * FROM card").unwrap();
    let card_iter = statement
        .query_map([], |row| {
            Ok(Card {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })
        .unwrap();

    // Just for debugging purposes
    // for card in card_iter {
    //     println!("Found card {:?}", card.unwrap());
    // }
}
