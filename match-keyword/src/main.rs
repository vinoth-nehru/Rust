
#![allow(unused)]

use std::io::{self, Read};
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::cmp::{Ordering, self};
use std::fs::File;
use std::ptr::null;

fn main() {
    let age :i32 = 19;
    match age {
        1..=17 => println!("YOu r a child"),
        18 => println!("You can vote"),
        19..=i32::MAX => println!("You are mature"),
        _ => println!("This is default"),
    };


    let my_age = 18;
    let vote_age = 18;
    match my_age.cmp(&vote_age){
        Ordering::Equal => println!("You gained the right to vote"),
        Ordering::Greater => println!("Yu can vote"),
        Ordering::Less => println!("You cannot vote"),
    };
}
