pub fn bubble_sort(array: &mut [i32]) {
    for i in 0..array.len() {
        for j in 0..array.len() - 1 - i {
            if array[j] > array[j + 1] {
                let x = array[j];
                array[j] = array[j + 1];
                array[j + 1] = x;
            }
        }
    }
}

fn qs(array: &mut [i32], lo: i32, hi: i32) {
    if lo > hi {
        return;
    }

    let pivot_idx = partition(array, lo, hi);
    qs(array, lo, pivot_idx - 1);
    qs(array, pivot_idx + 1, hi);
}

fn partition(array: &mut [i32], lo: i32, hi: i32) -> i32 {
    let pivot = array[hi as usize];
    let mut idx = lo - 1;

    let mut i = lo;
    while i < hi {
        if array[i as usize] <= pivot {
            idx += 1;
            let temp = array[i as usize];
            array[i as usize] = array[idx as usize];
            array[idx as usize] = temp;
        }
        i += 1;
    }
    idx += 1;
    array[hi as usize] = array[idx as usize];
    array[idx as usize] = pivot;

    idx
}

pub fn quick_sort(array: &mut [i32]) {
    qs(array, 0, array.len() as i32 - 1);
}
