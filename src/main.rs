///! Reimplementation of `FlushReload.c` in rust.
///! Taken from [Computer Security](https://tinyurl.com/tgs5tbw)
///! Listing 13.2
///!
///! Looks right, but doesn't work on my system (kernel 5.3.11)
///! Tried "mitigations=off", didn't work.
use core::arch::x86_64::_mm_clflush as mm_clflush;
use core::arch::x86_64::_rdtsc as rdtsc;
use std::ptr::read_volatile;
use std::sync::Mutex;
#[macro_use]
extern crate lazy_static;
const DELTA: usize = 1024;
const SECRET: u8 = 94;
lazy_static! {
    static ref DATA: Mutex<[u8; 256 * 4096]> = Mutex::new([0u8; 256 * 4096]);
}
fn flush_side_channel() {
    let mut data = DATA.lock().unwrap();
    for i in 0..256 {
        data[i * 4096 + DELTA] = 1;
    }
    for i in 0..256 {
        unsafe {
            mm_clflush(&data[i * 4096 + DELTA]);
        }
    }
}
fn get_secret() {
    let data = DATA.lock().unwrap();
    unsafe {
        read_volatile(&data[SECRET as usize * 4096 + DELTA]);
    }
}
fn reload_side_channel() {
    let data = DATA.lock().unwrap();
    let mut times = [0u64; 256];
    for i in 0..256 {
        unsafe {
            let t1 = rdtsc();
            read_volatile(&data[i * 4096 + DELTA]);
            times[i] = rdtsc() - t1;
        }
        println!("times[{}]={}", i, times[i]);
    }
    let (secret, time) = times.iter().enumerate().min_by_key(|x| x.1).unwrap();
    println!("secret {}, cycles {}", secret, time);
}

fn main() {
    flush_side_channel();
    get_secret();
    reload_side_channel();
}
