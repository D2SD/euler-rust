fn main() {
    let mut squared_sum = 0;
    let mut sum_squared = 0;

    for x in 1..101 {
        squared_sum += u32::pow(x, 2);
    }
    for x in 1..101 {
        sum_squared += x;
    }
    sum_squared = u32::pow(sum_squared, 2);
    println!("Sum squared: {}, squared sum: {}, difference: {}", sum_squared, squared_sum, sum_squared-squared_sum);

}
