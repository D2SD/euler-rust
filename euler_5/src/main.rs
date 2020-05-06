fn main() {
    let mut n = 2520;
    let limit = 21;
    let mut cont = 0;
    let mut exit = true;

    while exit {
        for i in 1..limit {
            //println!("cont: {}, dividing: {} / {}", cont, j, i);
            if n % i == 0 {
                cont += 1;
            } else {
                //println!("break!");
                cont = 1;
                break;
            }
            if cont == limit-1 {
                println!("found it!, the number is {}", n);
                exit = false;
            }
        }
        n += 1;
    }


}
