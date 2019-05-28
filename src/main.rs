use std::env;
use std::io::{self, Read, Write, BufReader, BufWriter};
extern crate base64;


const XOR_MASK: &str = "pbpPBP";

fn bad_encrpyt(buffer: &mut Vec<u8>) {
    // Base64 encoding
    let encoded_string = base64::encode(buffer).into_bytes();

    // XOR-ing and placing into the buffer
    *buffer = encoded_string.iter().zip(Vec::from(XOR_MASK).iter().cycle()).map(|(&x, &y)| x ^ y).collect();
}

fn bad_decrypt(buffer: &mut Vec<u8>) {
    let decoded_string: Vec<u8> = buffer.iter().zip(Vec::from(XOR_MASK).iter().cycle()).map(|(&x, &y)| x ^ y).collect();

    *buffer = base64::decode(&decoded_string).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = match args.len() {
        // Encrypt
        1 => 1,
        // Decrypt
        2 => 2,
        _ => 2
    };

    let stdin = io::stdin();
    let in_lock = stdin.lock();
    let mut in_reader = BufReader::new(in_lock);
    let mut input_buffer = Vec::new();

    match in_reader.read_to_end(&mut input_buffer) {
        Ok(_n_bytes) => if mode == 1 {
            bad_encrpyt(&mut input_buffer);
            drop(in_reader);
        } else {
            bad_decrypt(&mut input_buffer);
            drop(in_reader);
        },
        Err(error) => {
            eprintln!("[pbp] Error while reading from stdin: {}", error);
            return;
        },
    };

    let stdout = io::stdout();
    let out_lock = stdout.lock();
    let mut out_writer = BufWriter::new(out_lock);

    match out_writer.write_all(&input_buffer) {
        Ok(()) => {
            drop(out_writer);
        },
        Err(error) => {
            eprintln!("[pbp] Error while writing to stdout: {}", error);
            return;
        },
    };
}
