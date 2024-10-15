use crate::utils::*;
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static SORT_FUNCTIONS: Lazy<HashMap<&'static str, fn(&mut Vec<i32>)>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("selection_sort", selection_sort as fn(&mut Vec<i32>));
    m.insert("insertion_sort", insertion_sort as fn(&mut Vec<i32>));
    m.insert("bubble_sort", bubble_sort as fn(&mut Vec<i32>));
    m.insert("quick_sort", quick_sort as fn(&mut Vec<i32>));
    m.insert("merge_sort", merge_sort as fn(&mut Vec<i32>));
    m.insert("shell_sort", shell_sort as fn(&mut Vec<i32>));
    m.insert("heap_sort", heap_sort as fn(&mut Vec<i32>));
    // m.insert("bogo_sort", bogo_sort as fn(&mut Vec<i32>));
    // m.insert("bogobogo_sort", bogobogo_sort as fn(&mut Vec<i32>));
    m
});


/// Selection Sort:
/// Loops through array, looking for minimum element it can find.
/// Once at the end of the list, it places it into a "sorted section"
/// of the array, declaring that that part of the array is sorted and
/// moving on to loop again. This is done until the loop is completed.
pub fn selection_sort(array: &mut Vec<i32>)
{
    for sorted_len in 0..array.len() {
        let mut min_index = store_and_log(vec![sorted_len], "selection_sort").unwrap()[0];
        let mut min_value = array[min_index];
        for (index, value) in array[sorted_len..].iter().enumerate() {
            if *value < min_value {
                min_value = *value;
                min_index = sorted_len + index;
            }
        }
        if min_index != sorted_len {
            swap_and_log(array, sorted_len, min_index, "selection_sort").unwrap();
        }
    }
}

/// Insertion Sort:
/// Goes through array, finding out-of-place elements
/// and moving them backwards. Once it finds its place,
/// it continues forward in its progression until all
/// elements are in place.
pub fn insertion_sort(array: &mut Vec<i32>)
{
    for mut ind in 1..(array.len()) {
        // let temp = array[ind];
        let temp = array[store_and_log(vec![ind], "insertion_sort").unwrap()[0]]; // store_and_log
                                                                                         //  -> Result<Vec<usize>>
        while ind > 0 && array[ind-1] > temp {  // move all values gt temp over until value lt
                                                // is found
            swap_and_log(array, ind, ind-1, "insertion_sort").unwrap();
            ind -= 1;
        }
        // Insert
        array[ind] = temp; 
    }
}

/// Loops through array, finding the max-valued element.
/// As it sorts, it moves the element it finds to be the 
/// max up to that element along with it, helping place
/// the greater elements more towards the top of the array.
/// This makes it quicker to sort as it goes on.
/// Each iteration puts one more max element in place, shrinking
/// the size that the loop has to iterate over.
pub fn bubble_sort(array: &mut Vec<i32>)
{
    for i in 0..(array.len()) {
        let mut swapped = false;
        for j in 0..(array.len()-1-i) {
            if array[j] > array[j+1] {
                swap_and_log(array, j+1, j, "bubble_sort").unwrap();
                swapped = true;
            }
        }
        if !swapped {
            break;
        }

    }
}

/// Quick Sort:
/// Declares a pivot, splits array into two depending
/// on if values are less or greater than that pivot,
/// recombines them, then recursively does this until
/// the array is sorted.
pub fn quick_sort(array: &mut Vec<i32>)
{
    if array.len() <= 1 {
        return;
    }
    let pivot = array[0];
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for &val in array.iter().skip(1) {
        if val <= pivot {
            left.push(val);
        } else {
            right.push(val);
        }
    }

    quick_sort(&mut left);
    quick_sort(&mut right);

    array.clear();
    array.extend(left);
    array.push(pivot);
    array.extend(right);
}

/// Merge Sort:
/// Splits array recursively into two arrays until 1 or no elements in them,
/// combines arrays by comparing each element in the two arrays,
/// builds back up.
pub fn merge_sort(array: &mut Vec<i32>)
{
    if array.len() <= 1 {
        return;
    }

    let mut left: Vec<i32> = array[0..(array.len()/2)].to_vec();
    let mut right: Vec<i32> = array[(array.len()/2)..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    let (mut i, mut j) = (0,0);

    array.clear();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            array.push(left[i]);
            i += 1;
        } else {
            array.push(right[j]);
            j += 1;
        }
    }
    array.extend(left[i..].iter());
    array.extend(right[j..].iter());
}

pub fn shell_sort(array: &mut Vec<i32>)
{
    let mut gap = array.len()/2;

    while gap > 0 {
        for i in gap..array.len() {
            let temp = array[i];
            let mut j = i;
            while j >= gap && array[j - gap] > temp {
                array[j] = array[j - gap]; 
                swap_and_log(array, j, j - gap, "shell_sort").unwrap();
                j -= gap;
            }
            array[j] = temp;
        }
        gap /= 2;
    }
}

pub fn heap_sort(array: &mut Vec<i32>)
{
    for i in (0..array.len() / 2).rev() {
        heapify(array, array.len(), i);
    }

    for i in (1..array.len()).rev() {
        swap_and_log(array, 0, i, "heap_sort").unwrap();
        heapify(array, i, 0);
    }
}

// Bad Sorting Algorithms

pub fn bogo_sort(array: &mut Vec<i32>)
{
    while !is_sorted(array) {
        shuffle(array);
    }
}

pub fn bogobogo_sort(array: &mut Vec<i32>)
{
    if array.len() <= 1 {
        return;
    }

    bogobogo_sort(&mut array[0..(array.len()-1)].to_vec());

    bogo_sort(array)
}




