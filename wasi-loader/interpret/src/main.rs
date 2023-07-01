//! This interpreter converts a declarative wah/WASI configuration into a linear instruction
//! sequence to be interpreted by the `stage2-wasi.js` program.
//!
//! Please do not rely on any of its implementation details (i.e. output). But make use of its
//! format description as a schema (see [`config`][`config`]).

/// Defines the declarative configuration format.
pub mod config;

use std::{io::{Read, Write}, borrow::Cow};

const INST_SKIP: u32 = 1;
const INST_STRING: u32 = 2;
const INST_CONST: u32 = 4;
const INST_GET: u32 = 6;
const INST_SET: u32 = 7;
const INST_PREOPEN: u32 = 10;
const INST_UNZIP: u32 = 12;
const INST_SECTION: u32 = 13;

// Start of user-defined stack values.
const OPS: u32 = 14;
const ZIP_SECTION: &[u8] = b"wah_polyglot_stage3";
const ENV_DIR: &[u8] = b"dir";
const ENV_FDS: &[u8] = b"fds";
const ENV_DIR_ROOT: &[u8] = b".";

/// Location points which we have to rewrite before returning.
enum Relocation {
    /// An offset within the literals section.
    Literal(u32),
}

pub fn main() -> Result<(), std::io::Error> {
    let mut buffer = vec![];
    std::io::stdin().read_to_end(&mut buffer)?;

    // Here, parse the configuration and setup WASI.
    let mut instructions = vec![0u8; 0];
    let mut strings = StringSection::default();
    let mut relocations: Vec<(u32, Relocation)> =  vec![];

    #[rustfmt::skip]
    let mk_hello_world = &mut [
        INST_STRING, 2, 0, ZIP_SECTION.len() as u32,
        INST_SECTION, 1, OPS + 0,
        INST_UNZIP, 1, OPS + 1,
        INST_STRING, 2, 0, ENV_DIR.len() as u32,

        INST_SET, 3, 0, OPS + 3, OPS + 2,
        INST_STRING, 2, 0, ENV_FDS.len() as u32,
        INST_GET, 2, 0, OPS + 5,
        INST_CONST, 1, 3,

        INST_STRING, 2, 0, ENV_DIR_ROOT.len() as u32,
        INST_PREOPEN, 2, OPS + 8, OPS + 2,
        INST_SET, 3, OPS + 6, OPS + 7, OPS + 9,
    ];

    relocations.push((2, strings.push(ZIP_SECTION)));
    relocations.push((12, strings.push(ENV_DIR)));
    relocations.push((21, strings.push(ENV_FDS)));
    relocations.push((32, strings.push(ENV_DIR_ROOT)));

    let mk_hello_world = &mut mk_hello_world[..];
    instructions.extend_from_slice(bytemuck::cast_slice::<_, u8>(mk_hello_world));
    let string_offset = strings.encode(&mut instructions);

    for (location, reloc) in relocations {
        let bytes = location as usize * core::mem::size_of::<u32>();
        let bytes: &mut [u8; 4] = (&mut instructions[bytes..][..4]).try_into().unwrap();
        let Relocation::Literal(off) = reloc;
        *bytes = (string_offset + u32::from_ne_bytes(*bytes) + off).to_ne_bytes();
    }

    std::io::stdout().write_all(&instructions)?;
    Ok(())
}

#[derive(Default)]
struct StringSection<'st> {
    buffer: Vec<Cow<'st, [u8]>>,
    offset: u32,
}

impl<'st> StringSection<'st> {
    pub fn push(&mut self, data: impl Into<Cow<'st, [u8]>>) -> Relocation {
        let data = data.into();
        let offset = self.offset;

        self.offset += u32::try_from(data.len()).expect("Unhandled string offset, too much data");
        self.buffer.push(data);

        Relocation::Literal(offset)
    }

    pub fn encode(self, instructions: &mut Vec<u8>) -> u32 {
        let pre_start: u32 = u32::try_from(instructions.len()).expect("Unhandled strings offset, too much data");
        let post_skip = pre_start + 12;
        let post = post_skip + self.offset;
        let pad = 3 - (post + 3) & 0x3;
        
        instructions.extend_from_slice(bytemuck::cast_slice::<u32, u8>(&[INST_SKIP, 1, post+pad]));
        assert_eq!(instructions.len(), post_skip as usize);

        for part in &self.buffer {
            instructions.extend_from_slice(part);
        }

        assert_eq!(instructions.len(), post as usize);

        if pad > 0 {
            instructions.extend_from_slice(&[0; 4][..pad as usize]);
        }

        post_skip
    }
}
