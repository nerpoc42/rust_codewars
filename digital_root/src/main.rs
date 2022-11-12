fn digital_root(mut n: i64) -> i64 {
    let mut sum = 0;
    
    while n != 0 {
        sum = sum + n % 10;
        n = n / 10;
    }
    
    if (sum as f64).log10().floor() >= 1.0 { 
        digital_root(sum) 
    } else { 
        sum 
    }
}

fn main() {
    println!("{}", digital_root(16));
}
