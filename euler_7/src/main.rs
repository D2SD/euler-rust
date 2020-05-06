
fn is_prime(n: u32) -> bool {

    for i in 2..n {
        if n % i == 0 {
            return false
            }
        }
    true
    }

fn main() {
    let mut n_ = 2;
    let mut cont = 0;
    let target = 10001;
    let mut exit = true;

    while exit {
        //println!("is the number {} prime? {}", i, is_prime(i));
        if is_prime(n_) {
            cont +=1
        }
        if cont == target {
            println!("the {} primal number is {}", target, n_);
            exit = false;
        }
        n_ += 1;
    }
}
