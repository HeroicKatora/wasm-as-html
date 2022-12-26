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
        data: if args.edit {
            assert!(std::env::var_os("WAH_POLYGLOT_EXPERIMENTAL").is_some());
            include_bytes!("stage1-edit.js")
        } else {
            include_bytes!("stage1.js")
        },
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
        let name = args
            .zip_section_name
            .as_deref()
            .unwrap_or("wah_polyglot_stage2_data");

        encoder.section(&wasm_encoder::CustomSection {
            name,
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
    // Positional arguments
    /// The stage 2 loader payload, a JS module.
    ///
    /// Stage 0 refers to the necessary inline script block to take control of HTML processing,
    /// stage 1 to the built-in jump pad implemented as a separate Javascript custom section.
    ///
    /// The stage 2 payload is your module that gains control of execution and is invoked with a
    /// fake request that resolves to full WASM module, after the page has been replaced with the
    /// indicated `index.html`. The stage 1 will call its default export as
    ///
    /// stage2_module.default(Promise.resolve(new Response(wasmblob)))
    #[arg(name = "STAGE2_JS")]
    stage_2: PathBuf,
    /// The web assembly module to embed ourselves in, default stdin.
    wasm: Option<PathBuf>,

    // Options.
    /// A file to write the module to, default stdout.
    #[arg(short, long)]
    out: Option<PathBuf>,
    /// An HTML page to use when invoking the loader. Setup by the stage 1 loader. Defaults to an
    /// empty page that hides some garbage from processing the WASM module header.
    #[arg(short, long)]
    index_html: Option<PathBuf>,
    /// A zip file to attach.
    ///
    /// This file is added as a final section of the module (so its central archive is within the
    /// last 512 bytes).
    #[arg(short, long = "trailing-zip", alias = "zip")]
    zip: Option<PathBuf>,
    /// A customized section name to use for the final zip section.
    ///
    /// The section is named `wah_polyglot_stage2_data` by default.
    #[arg(long = "trailing-zip-section")]
    zip_section_name: Option<String>,

    // Experimental section.
    /// Experimental. Hot-reload when the WASM file changes.
    ///
    /// Details and use-case are not entirely fixed. Should we 'reboot' into the new stage0,
    /// stage1, or stage2? At the moment it calls the _old_ stage2 with the _new_ WASM data. This
    /// works with Yew Apps, for example.
    ///
    /// Must set the environment variable `WAH_POLYGLOT_EXPERIMENTAL` to use.
    #[arg(long, alias = "dev")]
    edit: bool,
}
