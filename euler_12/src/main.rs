
fn factors(input: f64) -> Vec<f64> {
    let n = input;
    let mut i = 1.0;
    let mut vec = vec![];
    while i <= n.sqrt() {
        if (n % i) == 0.0 {
            if n / i == i {
                vec.append(&mut vec![i]);
            } else {
                vec.append(&mut vec![i]);
                vec.append(&mut vec![n/i]);
            }
        }
        i += 1.0;
    }
    vec
}

fn main() {
    let mut sum:f64 = 0.0;
    let mut len = 0;
    let mut n:f64 = 1.0;
    let target = 500;

    while len < target {
        sum = n*(n+1.0)/2.0;
        len = factors(sum).len();
        n += 1.0;
    }
    println!("number: {}, triangle number (sum): {}, len: {}", n-1.0, sum, len);
}
