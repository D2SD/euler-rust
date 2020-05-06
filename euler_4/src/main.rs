//use std::str;
// TODO input a number

fn is_even(x: usize) -> bool {
    if x % 2 == 0 {
        true
    } else {
        false
    }
}

fn is_palimdrome(n: u32) -> bool {
    //let n = 10;
    let n_string = n.to_string();
    let bytes = n_string.as_bytes();
    let to_string = bytes.to_vec();
    let len = to_string.len();
    let mut cont = 0;

    //println!("params: string: {}, len: {}, n: {}, to_string: {:?}", n_string, len, n, to_string);

    if is_even(len) {
        for i in 0..len/2 {
            if to_string[i] == to_string[len-1-i] {
                cont += 1;
                continue;
            } else {
                //println!("not a palindrome.");
                break;
            }
        }
    } else {
        //println!("odd number");
        for i in 0..(len-1)/2 {
            if to_string[i] == to_string[len-1-i] {
                cont += 1;
                continue;
            } else {
                //println!("the number {} is not a palimdrome.", n);
                break;
            }
        }
    }

    if cont == len/2 {
        //println!("the number {} is a palimdrome!", n);
        true
    } else {
        false
    }
}

fn main() {
    //input
    //let mut line = String::new();
    //println!("Enter a number :");
    //let b1 = std::io::stdin().read_line(&mut line).unwrap();
    //let value = b1;
    let mut max = 0;
    let mut temp;
    let mut q = 0;
    let mut w = 0;
    for e in 100..999 {
        for j in 100..999{
            temp = e*j;
            if is_palimdrome(temp) && temp > max{
                max = temp;
                q = e;
                w = j;
            }
        }
    }
    println!("the biggest palindrome is: {}, the product of {} and {}.", max, q, w);
}
