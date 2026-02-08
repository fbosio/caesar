use std::env;

const NUMBER_LETTERS_ALPHABET: u8 = b'Z' - b'A' + 1;

fn main() {
    let mut args = env::args();

    args.next(); // Skip program name
    let key = get_key_from_arg(args.next());

    dbg!(key);

    let mut output = String::new();

    for word in args {
        for byte in word.bytes() {
            let byte = (upper(byte) - b'A' + key) % NUMBER_LETTERS_ALPHABET + b'A';
            let letter: char = byte.try_into().unwrap();
            output.push_str(&letter.to_string());
        }
    }

    println!("{output}"); 
}

fn get_key_from_arg(arg: Option<String>) -> u8 {
    // Key could be negative and errors are handled lazily here, to be improved
    let key: u8 = arg.unwrap().parse().unwrap();
    key % NUMBER_LETTERS_ALPHABET
}

fn upper(byte: u8) -> u8 {
    // Not all possible unicode characters are handled here, to be improved
    if (byte >= b'A') & (byte <= b'Z') {
        byte
    } else if (byte < b'A') | (byte > b'z') {
        b' ' + b'A'
    } else {
        byte + b'A' - b'a'
    }
}
