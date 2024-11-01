const HTML: &str = include_str!("example.html");
use std::io::Write as _;

use html_and_tar::{Entry, TarEngine};

fn main() {
    const HTMLTAG: &str = "<html";
    const NEEDLE: &str = "HERE_LIE_DRAGONS";

    let html: usize = {
        let start = HTML.find(HTMLTAG).expect("no html tag opened");
        let end = HTML[start..].find(">").expect("no html tag closed");
        start + end + 1
    };

    let where_to_insert = HTML.find(NEEDLE).unwrap() + NEEDLE.len() + 2;

    let mut seq_of_bytes: Vec<&[u8]> = vec![];

    let mut engine = TarEngine::default();
    let init = engine.start_of_file(HTML[..html].as_bytes(), where_to_insert);

    seq_of_bytes.push(init.header.as_bytes());
    seq_of_bytes.push(init.extra.as_slice());
    seq_of_bytes.push(HTML[init.consumed..where_to_insert].as_bytes());

    let data = engine.escaped_insert_base64(Entry {
        name: "example0",
        data: b"Hello, world!",
    });

    seq_of_bytes.push(data.padding);
    seq_of_bytes.push(data.header.as_bytes());
    seq_of_bytes.push(data.file.as_bytes());
    seq_of_bytes.push(data.data.as_slice());

    let data = engine.escaped_continue_base64(Entry {
        name: "InWonderland",
        data: b"Go ask Alice",
    });
    seq_of_bytes.push(data.padding);

    seq_of_bytes.push(data.header.as_bytes());
    seq_of_bytes.push(data.file.as_bytes());
    seq_of_bytes.push(data.data.as_slice());

    let end = engine.escaped_end(HTML.len() - where_to_insert);
    seq_of_bytes.push(end.padding);
    seq_of_bytes.push(end.header.as_bytes());

    seq_of_bytes.push(HTML[where_to_insert..].as_bytes());

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    for item in seq_of_bytes {
        stdout.write_all(item).unwrap();
    }
}
