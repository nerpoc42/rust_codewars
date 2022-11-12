fn sq_in_rect(mut lng: i32, mut wdth: i32) -> Option<Vec<i32>> {
    if lng == wdth {
        return None;
    }

    let mut squares = vec![];
    let mut max = if lng > wdth { &mut lng } else { &mut wdth };
    let mut min: &mut i32;
    
    while *max > 0 {
        (min, max) = if lng < wdth { (&mut lng, &mut wdth) } else { (&mut wdth, &mut lng) };
        squares.push(min.clone());
        *max = *max - *min;
    }

    Some(squares)
}

fn main() {
    println!("{:?}", sq_in_rect(3,5));
}
