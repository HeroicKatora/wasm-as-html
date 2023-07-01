use std::io::Read;
use zip::ZipArchive;

const STAGE3_JS: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/boot.mjs"));

pub fn main() -> Result<(), zip::result::ZipError> {
    let mut stdin = std::io::stdin();
    let mut data = vec![];
    stdin.read_to_end(&mut data)?;

    let wasm_binary = std::fs::read("/proc/self/exe")?;

    let data = std::io::Cursor::new(data);
    let mut archive = ZipArchive::new(data)?;

    std::fs::create_dir("/mnt")?;
    archive.extract("/mnt")?;
    std::fs::write("/proc/self/index.mjs", STAGE3_JS)?;

    Ok(())
}
