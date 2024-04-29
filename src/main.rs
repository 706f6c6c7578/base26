use std::io::{self, Read, Write};

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHABET_LEN: usize = 26;
const BYTE_VALUES: usize = 256;
const NOM: usize = 851;
const DENOM: usize = 500;

fn encode(src: &[u8], line_length: usize) {
    let mut src = src.to_owned();
    let mut out = String::new();
    let out_length = (src.len() * NOM + DENOM - 1) / DENOM;

    for i in 0..out_length {
        let mut acc = 0;
        for j in (0..src.len()).rev() {
            let full_val = (acc * BYTE_VALUES) + src[j] as usize;
            let full_val_mod = full_val % ALPHABET_LEN;
            src[j] = ((full_val - full_val_mod) / ALPHABET_LEN) as u8;
            acc = full_val_mod;
        }
        out.push(ALPHABET.chars().nth(acc).unwrap());

        if line_length > 0 && (i + 1) % line_length == 0 {
            out.push('\n');
        }
    }

    println!("{}", out);
}

fn decode(s: &str) {
    let s = s.replace("\n", ""); // Remove line breaks before decoding
    let out_length = (s.len() * DENOM + NOM - 1) / NOM;
    let mut out = Vec::with_capacity(out_length);

    let mut copy_s = s.as_bytes().to_vec();
    for _ in 0..out_length {
        let mut acc = 0;
        for i in (0..copy_s.len()).rev() {
            let value = acc * ALPHABET_LEN + (copy_s[i] as usize - 'A' as usize);
            copy_s[i] = (value / BYTE_VALUES) as u8 + 'A' as u8;
            acc = value % BYTE_VALUES;
        }
        out.push(acc as u8);
    }

    out.truncate(out.len() - 1);

    io::stdout().write_all(&out).expect("Error writing output");
    print!("");
}

fn main() {
    let mut input_data = Vec::new();
    io::stdin().read_to_end(&mut input_data).expect("Error reading input");

    let args: Vec<String> = std::env::args().collect();
    let mut line_length = 0;

    match args.len() {
        2 => {
            if args[1] == "-d" {
                decode(&String::from_utf8(input_data).unwrap().trim());
                return;
            } else {
                println!("Usage: {} [-d] [-l line_length] < infile > outfile", args[0]);
                return;
            }
        },
        3 => {
            if args[1] == "-l" {
                line_length = match args[2].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid line length");
                        return;
                    }
                };
            } else {
                println!("Usage: {} [-d] [-l line_length] < infile > outfile", args[0]);
                return;
            }
        },
        _ => {
            if args.len() > 1 {
                println!("Usage: {} [-d] [-l line_length] < infile > outfile", args[0]);
                return;
            }
        }
    }

    encode(&input_data, line_length);
}
