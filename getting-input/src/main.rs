#![allow(unused)]

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::cmp::Ordering;
use std::fs::File;


fn main() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting="Nice to meet you";
    io::stdin().read_line(& mut name)
        .expect("Didn't received input");
    println!("Hi {}! {}",name.trim_end(),greeting);
}
