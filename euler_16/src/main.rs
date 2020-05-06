
use std::convert::TryInto;

fn vec_sum(vec: std::vec::Vec<u16>) -> u16 {
    let mut sum = 0;
    for ele in vec {
        sum += ele;
    }
    sum
}

fn char_mul(a:u16, b:u16, c: u16) -> std::vec::Vec<u16> {
    let mul = a * b + c;
    let mut vec = vec![];
    if mul > 0 {
        vec.push(mul/10);
        vec.push(mul%10);
        //println!("multiplication: {:?}, a: {}, b: {}", vec, a, b);
        vec
    } else {
        vec.push(0);
        vec.push(mul);
        //println!("multiplication: {:?}, a: {}, b: {}", vec, a, b);
        vec
    }
}

fn vec_mul(a: std::vec::Vec<u16>, b: u16) -> std::vec::Vec<u16> {
    let mut a_;
    let mut b_;
    let mut reserva = 0;
    let mut result_vec = vec![];

    let vec_max_len:usize = a.len().try_into().unwrap();

    for i in 0.. vec_max_len {
        if i > 0 {
            a_ = a[i].to_string().parse::<u16>().unwrap();
            b_ = b;
            result_vec.push(char_mul(a_, b_, reserva)[1]);
            //println!("values update: {:?}, {}, {}", result_vec, b_, reserva);
            reserva = char_mul(a_, b_, reserva)[0];
        } else {
            a_ = a[i].to_string().parse::<u16>().unwrap();
            b_ = b;
            reserva = char_mul(a_, b_, 0)[0];
            //println!("values update: {:?}, {}, {}", result_vec, b_, reserva);
            result_vec.push(char_mul(a_, b_, 0)[1]);
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
    let a = 2;
    let mut vec_mul_result = vec![2];

    for _i in 1..1000 {
        vec_mul_result = vec_mul(vec_mul_result, a);
        //println!("resultado: {:?}", vec_mul_result);
    }
    //println!("{:?}", vec_sum(vec_mul_result));
    println!("{:?}", vec_mul_result);
    println!("{}", vec_sum(vec_mul_result));

}