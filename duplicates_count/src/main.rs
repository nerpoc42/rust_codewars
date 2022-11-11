use std::collections::HashMap;

fn count_duplicates(text: &str) -> u32 {
    let mut counter = HashMap::new();

    text.to_lowercase().chars().for_each(|c| {
        counter.entry(c).and_modify(|count| *count += 1).or_insert(1);
    });

    counter.values().fold(0, |dup_count, count| {
        if *count >= 2 { dup_count + 1 } else { dup_count }
    })
}

fn main() {
    println!("{}", count_duplicates("aBbcCcdeff"));
}
