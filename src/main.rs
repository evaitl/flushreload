///! Reimplementation of `FlushReload.c` in rust.
///! Taken from [Computer Security](https://tinyurl.com/tgs5tbw)
///! Listing 13.2
///!
///! Looks right, but doesn't work on my system (kernel 5.3.11)
///! Tried "mitigations=off", didn't work.
use core::arch::x86_64::_mm_clflush as mm_clflush;
use core::arch::x86_64::_rdtsc as rdtsc;
use std::ptr::read_volatile;
use std::boxed::Box;
const DELTA: usize = 1024;
const SECRET: u8 = 94;
type DataType=[u8;256*4096];
fn flush_side_channel(a: &mut Box<DataType>) {
    for i in 0..256 {
        a[i * 4096 + DELTA] = 1;
    }
    for i in 0..256 {
        unsafe {
            mm_clflush(&a[i * 4096 + DELTA]);
        }
    }
}
fn get_secret(a: &Box<DataType>) {
    unsafe {
        read_volatile(&a[SECRET as usize * 4096 + DELTA]);
    }
}
fn reload_side_channel(a: &Box<DataType>) {
    let mut times = [0u64; 256];
    for i in 0..256 {
        unsafe {
            let t1 = rdtsc();
            read_volatile(&a[i * 4096 + DELTA]);
            times[i] = rdtsc() - t1;
        }
        println!("times[{}]={}", i, times[i]);
    }
    let (secret, time) = times.iter().enumerate().min_by_key(|x| x.1).unwrap();
    println!("secret {}, cycles {}", secret, time);
}

fn main() {
    let mut data = Box::new([0u8;256*4096]);
    flush_side_channel(&mut data);
    get_secret(&data);
    reload_side_channel(&data);
}
