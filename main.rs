use std::{io::{Read}, iter::Successors, clone, borrow::Borrow};
/**
 * Good 'ol partition, but slower than the hoare partition :()
 */
pub fn partition(v: &mut [i32], lo: isize, hi: isize) -> isize {
    let pivot = v[hi as usize];
    let mut i = lo - 1;
    for j in lo..hi {
        if v[j as usize] <= pivot {
            i = i+1;
            v.swap(i as usize, j as usize);
        }
    }
    i = i+1;
    v.swap(i as usize, hi as usize);
    return i;
}
/**
 * 
 */
pub fn hoare_partition(v: &mut [i32], lo: usize, hi: usize) -> usize {
    median_pivot(v, lo, hi); //Position the median at the top
    let mut i = lo as i32 - 1; //Left pointer
    let mut j = hi as i32 + 1; //Right pointer
    let pivot = v[hi]; //Select the median as the pivot

    loop {
        while {i += 1; v[i as usize] < pivot}{} //Traverse the left pointer to the right as long as its < pivot
        while {j -= 1; v[j as usize] > pivot}{} //Traverse the right pointer to the left as long as its > pivot
        if i>=j{
            return j as usize; //Return the index of the pivot.
        }
        v.swap(i as usize, j as usize); //Swap the items between the pointers
    }
}

/**
 * Function to put the median of the low, high and middle value at the end position, V[hi]
 * For example
 * [3,2,1]
 * Step 1. 2 < 1, false: No change
 * Step 2. 1 < 3, true: swap 1 & 3 -> Result: [1,2,3]
 * Step 3. 2 < 3, true: swap 2 & 3 -> Result: [1,3,2]
 * Now the hi index contains the median of the three.
 * This helps to reduce cases of worst time complexity
 * Worst cases happend if the pivot is the smallest or largest value in the array which leads to a linked list bevhavior.
 */
pub fn median_pivot(v: &mut [i32], lo: usize, hi: usize) {
    let mid = (lo + hi)/2;
    if v[mid] < v[lo] {
        v.swap(lo, mid);
    }
    if v[hi] < v[lo] {
        v.swap(lo, hi);
    }
    if v[mid] < v[hi] {
        v.swap(mid, hi);
    }

}



/**
 * The recurisive part of the algorithm
 * Takes the whole list as input along with a start and end value for the sopecific partition to be looked at.
 * Switches too insertion sort if the partition size is less than 15, testing found that 15 was the best value to use.
 */
pub fn q_in_sort(v: &mut [i32], lo: usize, hi: usize) {
        if hi-lo < 15 { // No check for empty slice since it will already use insertion sort before that happends.
            in_sort(v, lo, hi); //Insertion sort
            return; //We can return since that part of the array is now sorted.
        }
            let p = hoare_partition(v, lo, hi);
            q_in_sort(v, lo, p); //Take the left side of the array from the partition, including the pivot
            q_in_sort(v, p+1, hi); //The right side of the array from the partition, excluding the pivot.
}

pub fn in_sort(v: &mut [i32], lo: usize, hi: usize) {
    for i in lo+1..hi+1 {
        let mut j = i;
        while j>0 && v[j-1] > v[j] {
            v.swap(j, j -1);
            j = j - 1;
        }
    }
}


fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input);

    let mut values: Vec<i32> = input
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let length = values.len() as isize;
    q_in_sort(&mut values, 0, length as usize-1);
    for i in values {
        print!("{} ", i);
        
    }
    println!("");
}
