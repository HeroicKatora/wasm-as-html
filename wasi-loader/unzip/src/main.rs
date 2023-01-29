use std::io::{Read, Write};
use zip::ZipArchive;

pub fn main() -> Result<(), zip::result::ZipError> {
    let mut stdin = std::io::stdin();
    let mut data = vec![];
    stdin.read_to_end(&mut data)?;

    let data = std::io::Cursor::new(data);
    let mut archive = ZipArchive::new(data)?;

    println!("{:?}", 0);
    let extract = archive.extract(".")?;
    println!("{:?}", 1);

    Ok(())
}
