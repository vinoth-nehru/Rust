#![allow(unused)]

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::cmp::Ordering;
use std::fs::File;


fn main() {
    enum Day{
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }

    impl Day{
        fn is_weekend(&self) -> bool{
            match self{
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }
    let today =Day::Monday;
    match today{
        Day::Monday => println!("Mon"),
        Day::Tuesday => println!("Tue"),
        Day::Wednesday => println!("Wed"),
        Day::Thursday => println!("Thu"),
        Day::Friday => println!("Fri"),
        Day::Saturday => println!("Sat"),
        Day::Sunday=> println!("Sun"),
    };

     println!("Is today a weekend {}",today.is_weekend());
}
