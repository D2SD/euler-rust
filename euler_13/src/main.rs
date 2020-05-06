
use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;
use std::convert::TryInto;

fn char_sum(a:u16, b:u16) -> std::vec::Vec<u16> {
    let sum = a + b;
    let mut vec = vec![];
    if sum > 0 {
        vec.push(sum/10);
        vec.push(sum%10);
        vec
    } else {
        vec.push(0);
        vec.push(sum);
        vec
    }
}

fn fill_cero(mut a: std::vec::Vec<u16>, num: usize) -> std::vec::Vec<u16> {
    for _i in 0..num {
        a.push(0)
    }
    a
}

fn vec_sum(mut a: std::vec::Vec<u16>, mut b: std::vec::Vec<u16>) -> std::vec::Vec<u16> {
    let mut a_;
    let mut b_;
    let mut reserva = 0;
    let mut result_vec = vec![];

    let vec_max_len_a:usize = a.len().try_into().unwrap();
    let vec_max_len_b:usize = b.len().try_into().unwrap();
    let vec_max_len = max(vec_max_len_a, vec_max_len_b);

    if vec_max_len_b > vec_max_len_a {
        a = fill_cero(a, vec_max_len_b-vec_max_len_a);
    } else if vec_max_len_a > vec_max_len_b {
        b = fill_cero(b, vec_max_len_a-vec_max_len_b);
    }

    for i in 0.. vec_max_len {
        if i > 0 {
            a_ = a[i].to_string().parse::<u16>().unwrap();
            b_ = b[i].to_string().parse::<u16>().unwrap();
            result_vec.push(char_sum(a_, b_ + reserva)[1]);
            reserva = char_sum(a_, b_ + reserva)[0];
        } else {
            a_ = a[i].to_string().parse::<u16>().unwrap();
            b_ = b[i].to_string().parse::<u16>().unwrap();
            reserva = char_sum(a_, b_)[0];
            result_vec.push(char_sum(a_, b_)[1]);
        }
    }
    if reserva > 0 {
        result_vec.push(reserva);
        result_vec
    } else {
        result_vec
    }
}

fn main() {
    let mut vec: Vec<String> = Vec::new();
    let filename = "large_number.txt";
    let mut temp = vec![];
    let mut rev_vec = vec![];

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    for line in contents.lines() {
        vec.push(line.to_string());
    }

    for line in &vec {
        for c in line.chars().rev() {
        temp.push(c.to_string().parse::<u16>().unwrap());
        }
        rev_vec.push(temp);
        temp = vec![];
    }

    let mut temp_1 = &rev_vec[0];
    let temp_2 = &rev_vec[1];
    let mut temp_sum;
    temp_sum = vec_sum(temp_1.to_vec(), temp_2.to_vec());

    for i in 2..rev_vec.len() {
        temp_1 = &rev_vec[i];
        temp_sum = vec_sum(temp_sum, temp_1.to_vec());
    }
    println!("{:?}", temp_sum);
}