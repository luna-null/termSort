use rand::{self, Rng};
use std::{collections::HashMap, env, fmt::{Display, Write}, fs::{self, File, OpenOptions}, io, path::PathBuf, time::{self, Instant}};
use csv::Writer;
use crate::sorting_algorithms::SORT_FUNCTIONS;

pub fn handle_sort(name: &str, array: &mut Vec<i32>) 
{
    if let Some(sort_func) = SORT_FUNCTIONS.get(name).copied() {
        let duration = timer(|| shuffle_and_sort(sort_func, array));
        println!("{}: {:?} \nTime: {} microseconds\n", name, array, duration);

    }
}

pub fn shuffle(array: &mut Vec<i32>) -> &mut Vec<i32>
{
    let mut rng = rand::thread_rng();
    for ind in 0..array.len() {
        let rand: usize = rng.gen_range(0..array.len());
        array.swap(ind, rand);
    }
    array
}

pub fn shuffle_and_sort<F>(sorting_algorithm: F, array: &mut Vec<i32>)
where
    F: Fn(&mut Vec<i32>),
{
    sorting_algorithm(shuffle(array));
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
    for i in 0..(array.len()-1) {
        if array[i] > array[i+1] {
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
        largest = left;
    }
    if right < length && array[right] > array[largest] {
        largest = right;
    }

    if largest != root {
        array.swap(root, largest);
        heapify(array, length, largest);
    }
}

pub fn swap_and_log(array: &mut Vec<i32>, i: usize, j: usize, filename: &str)
    -> io::Result<()>
{
    array.swap(i, j);

    let file = open_data_file(format!("{}", filename).as_str())?;

    // Write the record to the CSV file
    let row_vector: Vec<String> = vec![ "swap".to_string(), i.to_string(), j.to_string()];

    write_to_data(file, &row_vector)?;

    Ok(())
}

// Stores a vector of usizes from the array, and returns them in a vector
pub fn store_and_log<T: Display>(indices: Vec<T>, filename: &str)
    -> Result<Vec<T>, io::Error>
{

    let mut row_vector: Vec<String> = indices.iter().map(|x| x.to_string()).collect();
    row_vector.insert(0, "store".to_string());

    let file = open_data_file(filename)?;

    write_to_data(file, &row_vector)?;

    Ok(indices)
}

pub fn open_data_file(filename: &str) -> Result<File, io::Error> {
    let log_dir = PathBuf::from("logs");

    // Ensure the log directory exists
    if !log_dir.exists() {
        fs::create_dir_all(&log_dir)?;
    }

    // Create the full path for the CSV file
    let file_path = log_dir.join(format!("{}.csv", filename));

    // Open the CSV file in append mode
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;

    Ok(file)
}

pub fn write_to_data(file: File, row: &Vec<String>) -> io::Result<()>
{

    // Create a CSV writer
    let mut wtr = Writer::from_writer(file);

    // Write the record to the CSV file
    wtr.write_record(row)?;

    // Flush the writer to ensure all data is written
    wtr.flush()?;

    Ok(())
}
