#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::cmp::Ordering;
use std::fs::File;

fn main() {
    let num1 : u32 = 5;
    let mut num2 : u32 = 4;
    println!("5 + 4 = {}",num1 + num2);
    println!("5 - 4 = {}",num1 - num2);
    println!("5 * 4 = {}",num1 * num2);
    println!("5 / 4 = {}",num1 / num2);
    println!("5 % 4 = {}",num1 % num2);
    num2 += 1;
    println!("{}",num2);
    println!();
    // Ramdom
    let rand_num =rand::thread_rng().gen_range(0..101);
    println!("{}",rand_num);
}
