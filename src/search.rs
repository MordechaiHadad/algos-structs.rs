pub fn linear_search(haystack: &[i32], needle: i32) -> bool {
    for n in 0..haystack.len() {
        if haystack[n] == needle {
            return true;
        }
    }
    false
}

pub fn binary_search(haystack: &[i32], needle: i32) -> bool {
    let mut lo = 0;
    let mut hi = haystack.len();

    while lo < hi {
        let m: usize = lo + (hi - lo) / 2;
        let value = haystack[m];

        if value == needle {
            return true;
        } else if value > needle {
            hi = m;
        } else {
            lo = m + 1;
        }
    }
    false
}

pub fn two_crystal_balls(breaks: &[bool]) -> i32 {
    let jump_amount = f32::sqrt(breaks.len() as f32) as i32;
    let mut i = jump_amount;

    while i < breaks.len() as i32 {
        if breaks[i as usize] {
            break;
        }
        i += jump_amount;
    }
    i -= jump_amount;

    let mut j = 0;
    while j < jump_amount && i < breaks.len() as i32 {
        if breaks[i as usize] {
            return i;
        }

        i += 1;
        j += 1;

    }
    -1
}
