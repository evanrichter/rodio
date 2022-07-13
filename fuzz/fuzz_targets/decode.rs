#![no_main]
use libfuzzer_sys::fuzz_target;
use rodio::Decoder;

fuzz_target!(|data: Vec<u8>| {
    fuzz(data);
});

fn fuzz(data: Vec<u8>) -> Option<()> {
    let data = std::io::Cursor::new(data);
    let data = std::io::BufReader::new(data);
    let _source = Decoder::new(data).ok()?;

    None
}
