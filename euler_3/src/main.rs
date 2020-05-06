fn main() {
    let mut n: u64 = 600851475143 ;
    let ori = n;
    let mut div = 2;
    let mut vec = vec![];

    while n > 1 {
        if (n % div) == 0 {
            vec.append(&mut vec![div]);
            n /= div
        }
        else {
            div += 1;
        }
    }

    println!("{:?}", vec);

    // find max value
    let mut max = 0;
    for ele in vec {
        if ele > max {
            max = ele;
        }
    }
    println!("max factor of {} is: {}", ori, max);

}
