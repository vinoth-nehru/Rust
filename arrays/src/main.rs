#![allow(unused)]

use std::io::{self, Read};
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::cmp::{Ordering, self};
use std::fs::File;
use std::ptr::null;

fn main() {
    let arr1 =[1,2,3,4,5];
    println!("1st ele: {}",arr1[0]);
    println!("Length : {}",arr1.len());

    let nums =[1,2,3,4,5,6,7,8,9];
    let mut i = 0;
    loop{
        if(nums[i] % 2 == 0){
            i+=1;
            continue;
        }
        if(nums[i] == 9){
            break;
        }
        println!("ODD VAL : {}", nums[i]);
        i+=1;
    }

    let mut j = 0;
    print!("Array values are : ");
    while j < nums.len() {
        print!("{} ",nums[j]);
        j+=1;
    }
    println!();
    for val in nums.iter() {
        print!("{} ",val);
    }
}
