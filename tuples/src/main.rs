#![allow(unused)]

use std::io::{self, Read};
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::cmp::Ordering;
use std::fs::File;


fn main() {
    let mut my_tuple = (1,2,3,4,5);
    let tup: (i32,String,f32) = (50,"Vinoth".to_string(),5_000_000.0);
    //Without ref
    println!("My tuple name: {}",tup.1);
    let (v1,v2,v3) = tup;
    println!("V1: {}",v1);

    //With ref -- comment line 14 and 15
        // let (ref x1,ref x2,ref x3) = tup;
        // println!("My tuple name: {}",tup.1);
        // println!("V1: {}",x1);
}
