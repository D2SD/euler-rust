
//solution = 142913828922

fn is_prime(n: u64) -> bool {

    for i in 2..n {
        if n % i == 0 {
            return false
            }
        }
    true
    }

fn main() {
    let mut sum: u64 = 0;
    let mut n_ = 2;
    let target = 2000000;
    let mut exit = true;

    while exit {
        //println!("is the number {} prime? {}", i, is_prime(i));
        if is_prime(n_) {
            sum += n_;
        }
        if n_ == target {
            println!("the sum is {}", sum);
            exit = false;
        }
        n_ += 1;
        //println!("n: {}", n_);
        if n_ % 10000 == 0 {
            println!("iteration: {}", n_);
        }
    }
}