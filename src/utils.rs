use crate::{
    sorting_algorithms::SORT_FUNCTIONS,
    file_handling::{shuffle_and_log, swap_and_log},
};
use std::{
    str::FromStr,
    time::Instant,
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Instruction
{
    SHUFFLE,
    STORE,
    SWAP,
}

impl FromStr for Instruction
{
    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, Self::Err>
    {
        match input {
            "SHUFFLE"   => Ok(Instruction::SHUFFLE),
            "STORE"     => Ok(Instruction::STORE),
            "SWAP"      => Ok(Instruction::SWAP),
            _ => Err(()),
        }
    }
}

/// For handling each sorting function
pub fn handle_sort(sort_func_str: &str, array: &mut Vec<i32>)
{
    let sort_func = SORT_FUNCTIONS.get(sort_func_str).unwrap();

    shuffle_and_log(array, sort_func_str).unwrap();

    let duration = timer(|| {
        sort_func(array);
    });

    println!(
        "{}: {:?} \nTime: {} microseconds\n",
        sort_func_str, array, duration
    );
}

fn shuffle_and_sort<'a, F>(sorting_func_str: &str, array: &'a mut Vec<i32>) -> &'a mut Vec<i32>
where
    F: Fn(&mut Vec<i32>),
{
    // sort_function(shuffled_array)
    SORT_FUNCTIONS[sorting_func_str](shuffle_and_log(array, sorting_func_str).unwrap());
    array
}

pub fn timer<F, T>(function: F) -> u128
where
    F: FnOnce() -> T,
{
    let start_time = Instant::now();
    function();
    let duration = start_time.elapsed();

    duration.as_micros()
}
pub fn is_sorted(array: &mut Vec<i32>) -> bool
{
    let mut in_order: bool = true;
    for i in 0..(array.len() - 1) {
        if array[i] > array[i + 1] {
            in_order = false;
        }
    }
    in_order
}

pub fn heapify(array: &mut Vec<i32>, length: usize, root: usize)
{
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < length && array[left] > array[largest] {
        swap_and_log(array, largest, left, "heap_sort");
    }
    if right < length && array[right] > array[largest] {
        swap_and_log(array, largest, right, "heap_sort");
    }

    if largest != root {
        array.swap(root, largest);
        heapify(array, length, largest);
    }
}
