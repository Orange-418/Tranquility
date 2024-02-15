//When We Set An Intention, Somehow The World Seems To Magically Come To Meet Us.
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use clap::{App, Arg};
use flate2::write::GzEncoder;
use flate2::Compression;
use rand::{rngs::OsRng, RngCore};
use std::fs::File;
use std::io::{self, Read, Write};
use hex;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn compress(input: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input)?;
    encoder.finish().map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    cipher.encrypt_vec(data)
}

fn main() -> io::Result<()> {
    let matches = App::new("Tranquility Packer")
        .version("1.0")
        .about("Compresses and encrypts files")
        .arg(Arg::with_name("source")
            .short("s")
            .long("source")
            .takes_value(true)
            .required(true)
            .help("Source file to compress and encrypt"))
        .arg(Arg::with_name("target")
            .short("t")
            .long("target")
            .takes_value(true)
            .required(true)
            .help("Target file for the encrypted output"))
        .get_matches();

    let source_path = matches.value_of("source").unwrap();
    let target_path = matches.value_of("target").unwrap();

    let mut file = File::open(source_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let compressed = compress(&contents)?;

    let mut key = [0u8; 32];
    let mut iv = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    OsRng.fill_bytes(&mut iv);
    let encrypted = encrypt(&compressed, &key, &iv);

    let mut output_data = iv.to_vec();
    output_data.extend_from_slice(&encrypted);

    let mut output = File::create(target_path)?;
    output.write_all(&output_data)?;

    let key_file_name = format!("{}.key", target_path);
    let mut key_file = File::create(key_file_name)?;
    write!(key_file, "{}", hex::encode(key))?;

    Ok(())
}
