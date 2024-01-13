#![allow(unused)]

use std::io::{self, Read};
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::cmp::Ordering;
use std::fs::File;


fn main() {
    let mut str1 =String::new();
    str1.push('H');
    str1.push_str(" word");
    for words in str1.split_whitespace() {
        println!("{} ",words);
    }


    let str2= str1.replace('H', "Another");
    println!("{}",str2);

    let s=String::from("v i n o t t h");
    let mut vec: Vec<char> =s.chars().collect();
    vec.sort();
    vec.dedup();
    for cahar in vec {
        println!("{} ",cahar);
    }


    let str3="Random String";
    let mut s2=str3.to_string();
    println!("{}", s2);
    println!("{}", str3);
    let byt_arr =s2.as_bytes();
    let s3 =&s2[0..6];
    println!("{}", s3);
    println!("{}",s3.len());
    s2.clear();

    let s4 =String::from("a just some text");
    let s5 =String::from("on vs code");
    let s6=s4 + &s5;
    for ch in s6.bytes(){
        println!("{} ",ch);
    }
}
