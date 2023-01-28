#[link(wasm_import_module = "wah_wasi")]
extern "C" {
    fn length() -> usize;
    fn get(ptr: usize);
    fn put(ptr: usize, len: usize);
}

const INST_SKIP: u32 = 1;
const INST_STRING: u32 = 2;

#[no_mangle]
pub fn configure() {
    let len = unsafe { length() };

    let mut buffer = vec![0; len];
    unsafe { get(buffer.as_mut_ptr() as usize) };

    // Here, parse the configuration and setup WASI.
    let mut instructions = vec![0u8; 0];
    // string/2(28, 13)
    // skip/1(13)
    let mk_hello_world = &[INST_STRING, 2, 28, 13, INST_SKIP, 1, 13];
    instructions.extend_from_slice(bytemuck::bytes_of(mk_hello_world));
    instructions.extend_from_slice(b"Hello, world");

    unsafe { put(instructions.as_ptr() as usize, instructions.len()) };
}
