fn main() {

    let mut sum: u32;
    let mut even_sum: u32 = 0;
    let mut fn_1 = 1;
    let mut fn_2 = 2;
    

    while fn_1 + fn_2 < 4000000 {

        sum = fn_1 + fn_2;
        fn_1 = fn_2;
        fn_2 = sum;

        if sum % 2 == 0 {
            even_sum += sum;
        }
        // missed the initial 2
        //println!("sum: {}, even sum: {}", sum, even_sum+2);
    }

    println!("even sum: {}", even_sum+2);

}
