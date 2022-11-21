fn max_sequence(seq: &[i32]) -> i32 {
    let mut max_sum = 0;
    let length = seq.len();

    for i in 0..length {
        let mut sum = 0;

        for l in i..length {
            sum = sum + seq[l];

            if sum > max_sum {
                max_sum = sum;
            }
        }
    }

    max_sum
}

fn main() {
    println!("{}", max_sequence(&[2, -1]));
}
