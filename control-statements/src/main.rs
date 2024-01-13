#![allow(unused)]

use std::io::{self, Read};
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::cmp::Ordering;
use std::fs::File;


fn main() {
    let age: i32 = 18;
    if(age == 18){
        println!("Important Birthday and you can vote");
    }
    else if(age<18){
        println!("You r a minor");
    }
    else{
        println!("You r matured and can vote");
    }


    //Ternary operator 
    println!("Can I vote ?. Please Enter your age:");
    let mut my_age = String::new() ;
    io::stdin().read_line(&mut my_age)
        .expect("Age is not entered.");
    let age_num: i32 = my_age.trim().parse()
        .expect("Not converted to Int");
    let can_vote = if age_num >= 18 { "Success" } else { "Not_Success" } ;
    if can_vote=="Success"{
    println!(" You are {} and you can vote",my_age.trim_end());
    }
    else{
        println!("You are {} and you cannot vote",my_age.trim_end());
    }
    

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
