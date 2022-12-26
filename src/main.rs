use clap::Parser;
use std::{io::Read, io::Write, path::PathBuf};

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let stage_2 = std::fs::read(&args.stage_2)?;
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
        name: "wah_polyglot_stage0",
        // Html designed to terminate processing into further WASM sections. This is the only
        // section that needs to be placed specifically at the start. All other sections are then
        // parsed from the module.
        data: include_bytes!("stage0.html"),
    });

    // The actual (document) loader that prepares inputs and control for stage 2.
    encoder.section(&wasm_encoder::CustomSection {
        name: "wah_polyglot_stage1",
        data: include_bytes!("stage1.js"),
    });

    if let Some(index) = &args.index_html {
        let index_html = std::fs::read(index)?;

        encoder.section(&wasm_encoder::CustomSection {
            name: "wah_polyglot_stage1_html",
            data: &index_html,
        });
    }

    encoder.section(&wasm_encoder::CustomSection {
        name: "wah_polyglot_stage2",
        data: &stage_2,
    });

    for section in parser.parse_all(&wasm) {
        if let Some((id, data_range)) = section.map_err(parse_err)?.as_section() {
            encoder.section(&wasm_encoder::RawSection {
                id,
                data: &wasm[data_range],
            });
        }
    }

    if let Some(zip_file) = &args.zip {
        let zip_data = std::fs::read(zip_file)?;

        encoder.section(&wasm_encoder::CustomSection {
            name: "wah_polyglot_stage1_data",
            data: &zip_data,
        });
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
    /// The stage 2 loader payload.
    ///
    /// Stage 0 refers to the necessary inline script block to take control of HTML processing,
    /// stage 1 to the built-in jump pad implemented as a separate Javascript custom section. The
    /// stage 2 payload is a module that gains control of execution and is invoked with a fake
    /// request that resolves to full WASM module, after the page has been replaced with the
    /// indicated `index.html`.
    stage_2: PathBuf,
    /// The web assembly module to embed ourselves in, default stdin.
    wasm: Option<PathBuf>,
    /// A zip file to attach.
    #[arg(short, long)]
    zip: Option<PathBuf>,
}
