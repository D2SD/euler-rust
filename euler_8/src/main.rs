
//use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut vec = vec![];
    let filename = "digits_1000.txt";

    //println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    //println!("With text:\n{}", contents);

    for letter in contents.chars() {
        if letter != '\n' && letter != '\r' {
            vec.push(letter);
        }
    }

    let n: usize;
    let rest;
    let slice;
    let div;
    let mut max = 0;
    let mut temp: u64 = 1;
    let mut temp_2: u64;

    n = vec.len();
    slice = 13;
    rest = n % slice;
    div = (n-rest) / slice;
    //println!("n: {}, slice: {}, rest times: {}, div: {}", n, slice, rest, div);

    for i in 0..n-slice-rest {
        for j in 0+i..slice+i {
            temp_2 = vec[j].to_string().parse().unwrap();
            temp = temp*temp_2;
            //println!("temp: {}, temp_2: {}", temp, temp_2);
        }
        if temp > max {
            max = temp;
        }
        temp = 1;
    }

    println!("{}", max);

}
