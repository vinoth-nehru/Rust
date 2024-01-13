#![allow(unused)]

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::cmp::Ordering;
use std::fs::File;


fn main() {
    // Always use caps for constant
    const ONE_MILLION: u32 =1_000_000;
    const PI:f32 = 3.141592;
    // Variable with same name but different datatype is called SHADOWING
    let age="47";
    let mut age: u32 = age.trim().parse()
        .expect("Age is not a number");
    age+=1;
    println!("Iam {} and I have ₹{}", age,ONE_MILLION);


    println!("MAX u16: {}",u16::MAX);
    println!("MAX u32: {}",u32::MAX);
    println!("MAX u64: {}",u64::MAX);
    println!("MAX u128: {}",u128::MAX);
    println!("MAX usize: {}",usize::MAX);

    println!("MAX f32: {}",f32::MAX);
    println!("MAX f64: {}",f64::MAX);

    let is_true = true;
    let is_false = false;

    let my_char='v';

    let num1:f32 = 1.111111111111111111;
    let num2:f64 = 1.111111111111111111;
    println!("f32: {}",num1+0.111111111111111111);
    println!("f64: {}",num2+0.111111111111111111);

    //Type casting 

    let int_u8 :u8 =5;
    let int1_u8 :u8 =4;
    let int_u32 :u32 =int1_u8 as u32 + int_u8 as u32;   
    print!("{}", int_u32); 
}
