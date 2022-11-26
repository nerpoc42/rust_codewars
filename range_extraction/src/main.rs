pub fn range_extraction(a: &[i32]) -> String {
    let mut range_str = String::new();

    let mut i = 0;

    while let Some(start) = a.get(i) {
        let mut prev = start;
        i = i + 1;

        while let Some(num) = a.get(i) {
            if *num != prev + 1 {
                break;
            }

            i = i + 1;
            prev = num;
        }

        range_str = format!("{}{},", range_str, match start == prev {
            true => format!("{}", start),
            false => format!("{}{}{}", start, if prev - start == 1 { ',' } else { '-' }, prev)
        });
    }

    range_str.pop();

    range_str
}

fn main() {
    println!("{}", range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]));
}
