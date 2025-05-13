use std::time::SystemTime;

fn fibonacci(k: u32) -> u128 {
    if k == 0{
        return 0;
    }
    if k == 1{
        return 1;
    }
    return fibonacci(k - 2) + fibonacci(k - 1);
}
fn main() {
    let overall_start = SystemTime::now();
    for k in 0..=49 {
        let start = SystemTime::now();
        let result = fibonacci(k);
        let duration = start.elapsed().expect("Time measurement failed");

        println!("fibonacci({}) = {}, Time: {:?}", k, result, duration);
    }

    let overall_duration = overall_start.elapsed().expect("Time measurement failed");
    println!("Total execution time: {:?}", overall_duration);
}
