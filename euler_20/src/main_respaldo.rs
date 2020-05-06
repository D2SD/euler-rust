
use std::convert::TryInto;
use std::cmp::max;

/*fn vec_values_sum(vec: std::vec::Vec<u16>) -> u16 {
    let mut sum = 0;
    for ele in vec {
        sum += ele;
    }
    sum
}*/

fn fill_cero(mut a: std::vec::Vec<u16>, num: usize) -> std::vec::Vec<u16> {
    for _i in 0..num {
        a.push(0)
    }
    a
}

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

/*fn char_mul(a:u16, b:u16, c: u16) -> std::vec::Vec<u16> {
    let mul = a * b + c;
    let mut vec = vec![];
    println!("{}, {}, {}", a, b, c);
    // refactor this with a for loop with the lenght of the chain? nested loop? recursive?
    if mul > 0 {
        vec.push(mul/10);
        vec.push(mul%10);
        println!("multiplication: {:?}, a: {}, b: {}", vec, a, b);
        vec
    } else {
        vec.push(0);
        vec.push(mul);
        println!("multiplication: {:?}, a: {}, b: {}", vec, a, b);
        vec
    }
}*/

/*fn vec_mul(a: std::vec::Vec<u16>, b: u16) -> std::vec::Vec<u16> {
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
}*/

fn main() {
    /*let mut n_factorial = 1;
    for i in (1..n).rev() {
        n_factorial *= i;
        println!("{}", n_factorial);
    }*/

    // continue
    let n = vec![1, 0, 0];
    let mut vec_mul_result = vec![0, 0, 1];

    /*for i in (1..n).rev() {
        vec_mul_result = vec_mul(vec_mul_result, i);
        println!("resultado: {:?}", vec_mul_result);
    }*/
    //println!("{:?}", vec_sum(vec_mul_result));
    //println!("{:?}", vec_mul_result);
    //println!("{}", vec_sum(vec_mul_result));

    // WORKING
    let a = vec![9, 9, 9, 5];
    let b = vec![3];
    let mut result = vec![];
    let mut temp_result = vec![];
    let mut mul;
    let mut reserva = 0;
    let mut cont = 1;
    //println!("{:?}", char_mul(a, b, 0));
    for ele in (0..a.len()).rev() {
        for leel in (0..b.len()).rev() {
            //println!("first loop: {}, second loop: {}", a[ele], b[leel]);
            mul = a[ele] * b[leel] + reserva;
            //println!("{}", mul);
            if mul > 0 {
                reserva = mul/10;
                result.push(mul%10);
            } else {
                result.push(0);
                reserva = mul/10;
                //println!("{}", mul%10);
            }
        }
        if reserva > 0 {
            result.push(reserva);
            reserva = 0;
        }
        if cont > 1 {
            temp_result = vec_sum(temp_result, result);
            result = vec![];
            //println!("temp: {:?} A", temp_result);
            for i in 0..cont {
                result.push(0);
            }
        } else {
            temp_result = result;
            //println!("init: {:?} B", temp_result);
            result = vec![];
            for i in 0..cont {
                result.push(0);
            }
        }
        cont += 1
    }
    println!("{:?}", temp_result);
}
