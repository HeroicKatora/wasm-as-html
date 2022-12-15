use clap::Parser;
use std::{io::Read, io::Write, path::PathBuf};

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let stage1 = std::fs::read(&args.stage_1)?;
    let wasm = match &args.wasm {
        None => {
            let mut stdin = std::io::stdin();
            let mut data = vec![];
            stdin.read_to_end(&mut data)?;
            data
        }
        Some(path) => std::fs::read(path)?,
    };

    let parser = wasmparser::Parser::default();
    let mut encoder = wasm_encoder::Module::new();

    encoder.section(&wasm_encoder::CustomSection {
        name: "polyglot_stage0",
        data: include_bytes!("stage0.html"),
    });

    encoder.section(&wasm_encoder::CustomSection {
        name: "polyglot_stage1",
        data: include_bytes!("stage1.js"),
    });

    if let Some(index) = &args.index_html {
        let index_html = std::fs::read(index)?;

        encoder.section(&wasm_encoder::CustomSection {
            name: "polyglot_stage1_html",
            data: &index_html,
        });
    }

    encoder.section(&wasm_encoder::CustomSection {
        name: "polyglot_stage2",
        data: &stage1,
    });

    for section in parser.parse_all(&wasm) {
        if let Some((id, data_range)) = section.map_err(parse_err)?.as_section() {
            encoder.section(&wasm_encoder::RawSection {
                id,
                data: &wasm[data_range],
            });
        }
    }

    let wasm = encoder.finish();

    match &args.out {
        None => {
            let mut stdout = std::io::stdout();
            stdout.write_all(&wasm)?;
        }
        Some(path) => {
            std::fs::write(path, &wasm)?;
        }
    }

    Ok(())
}

fn parse_err(_: wasmparser::BinaryReaderError) -> std::io::Error {
    todo!()
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    out: Option<PathBuf>,
    #[arg(short, long)]
    index_html: Option<PathBuf>,
    /// The stage 1 payload.
    stage_1: PathBuf,
    /// The web assembly module to embed ourselves in, default stdin.
    wasm: Option<PathBuf>,
}
