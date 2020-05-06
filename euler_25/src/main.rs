

use std::convert::TryInto;
use std::cmp::max;

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

fn vect_rev(a: &std::vec::Vec<u16>) -> std::vec::Vec<u16> {
    let vec = a;
    let mut b = vec![];
    let len = vec.len();
    for ele in (0..len.try_into().unwrap()).rev() {
        b.push(vec[ele]);
    }
    b
}

fn main() {
    
    let mut cont = 3;
    let mut result;
    let mut fn_1 = vec![1];
    let mut fn_2 = vec![1];
    let mut len = 0;


    result = vec_sum(vect_rev(&fn_1), vect_rev(&fn_2));

    while len < 1000 {

        result = vec_sum(vect_rev(&fn_1), vect_rev(&fn_2));
        len = result.len();
        //println!("len: {}, id: {}, result: {:?}", len, cont, vect_rev(&result));
        fn_1 = fn_2;
        fn_2 = vect_rev(&result);
        cont += 1;
    }
    println!("len: {}, id: {}, result: {:?}", len, cont-1, vect_rev(&result));

}