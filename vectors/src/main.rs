#![allow(unused)]

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::cmp::Ordering;
use std::fs::File;


fn main() {
    let vec1: Vec<i32> =Vec::new();
    let mut vec2: Vec<i32> =vec![1,2,3,4];
    vec2.push(5);
    println!("VEC ! : {}",vec2[0]);
    let sec =&vec2[1];;
    match vec2.get(1) {
        Some(sec) => println!("2nd :{}",sec),
        None => println!("No second value"),
    }
    for i in &mut vec2{
        *i *=2;
    }
    for i in & mut vec2{
        print!("{} ",i);
    }
    println!("VEC LEN:{} ",vec2.len());
    println!("POP :{:?}",vec2.pop());
}
