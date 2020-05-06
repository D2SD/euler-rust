
fn is_even(number: u64) -> bool {
    if number % 2 == 0 {
        true
    } else {
        false
    }
}

fn colltz(number: u64) -> std::vec::Vec<u64> {
    let mut n = number;
    let mut chain = vec![];
    chain.push(n);
    while n != 1 {
        if is_even(n) {
            n /= 2;
        } else {
            n = 3*n + 1;
        }
        chain.push(n);
    }
    chain
}

fn main() {
    let mut n = 0;
    let mut max = 0;
    let mut len;
    for i in (1..1000000).rev() {
        len = colltz(i).len();
        if len > max {
            n = i;
            max = len;
        }
    }

    println!("max len: {}, value: {}", max, n);
}
