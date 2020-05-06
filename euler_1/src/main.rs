
fn main() {

    let mut sum = 0;

    for x in 0..1000 {

        if x % 3 == 0 {
            //println!("multiplo de 3: {}", x);
            sum += x;
        } 
        else if x % 5 == 0 {
            //println!("multiplo de 5: {}", x);
            sum += x;
        }
        else {
            continue
        }
    }

    println!("suma: {}", sum);
}