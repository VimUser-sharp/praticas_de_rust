fn main() {
    println!("{:.2}", simple_calc(11, 97));
}

fn simple_calc(first_number: i64, second_number: i64) -> f64 {
    return (first_number as f64 + second_number as f64) / 2 as f64;
}
