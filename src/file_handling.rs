use std::{collections::HashMap, error::Error, fmt::Display, fs::{self, File, OpenOptions}, i32, io, num::ParseIntError, path::PathBuf};

use csv::{self, ReaderBuilder, Result, Writer};
use rand::Rng;

use crate::Instruction;


pub fn shuffle_and_log<'a>(
    array: &'a mut Vec<i32>,
    filename: &str,
) -> Result<&'a mut Vec<i32>>
{
    let mut rng = rand::thread_rng();
    for ind in 0..array.len() {
        let rand: usize = rng.gen_range(0..array.len());
        array.swap(ind, rand);
    }
    let file = open_write_data_file(filename)?;
    let mut row_vector: Vec<String> = vec!["SHUFFLE".to_string()];
    row_vector.extend(array.iter().map(|x| x.to_string()));
    write_to_data(file, &row_vector)?;

    Ok(array)
}
pub fn swap_and_log(
    array: &mut Vec<i32>,
    i: usize,
    j: usize,
    filename: &str,
) -> Result<(usize, usize)>
{
    array.swap(i, j);
    let file = open_write_data_file(format!("{}", filename).as_str())?;
    let row_vector: Vec<String> = vec!["SWAP".to_string(), i.to_string(), j.to_string()];
    write_to_data(file, &row_vector)?;

    Ok((i, j))
}

// Stores a vector of usizes from the array, and returns them in a vector
pub fn store_and_log<T: Display>(indices: Vec<T>, filename: &str) -> Result<Vec<T>>
{
    let mut row_vector: Vec<String> = indices.iter().map(|x| x.to_string()).collect();
    row_vector.insert(0, "STORE".to_string());

    let file = open_write_data_file(filename)?;

    write_to_data(file, &row_vector)?;

    Ok(indices)
}

pub fn open_read_data_file(filename: &str) -> Result<File>
{
    let log_dir = PathBuf::from("logs");
    if !log_dir.exists() {
        fs::create_dir_all(&log_dir)?;
    }
    let file_path = log_dir.join(format!("{}.csv", filename));
    let file = OpenOptions::new().read(true).open(file_path)?;
    Ok(file)
}

pub fn open_write_data_file(filename: &str) -> Result<File>
{
    let log_dir = PathBuf::from("logs");

    if !log_dir.exists() {
        fs::create_dir_all(&log_dir)?;
    }
    let file_path = log_dir.join(format!("{}.csv", filename));
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;
    Ok(file)
}

pub fn write_to_data(file: File, row: &Vec<String>) -> io::Result<()>
{
    let mut wtr = Writer::from_writer(file);
    wtr.write_record(row)?;
    wtr.flush()?;
    Ok(())
}

pub fn read_from_data(
    file: File,
) -> Result<(Vec<i32>, HashMap<Instruction, Vec<u16>>)>
{
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_reader(file);
    
    let mut records = rdr.records();

    // 1. Read the first row as the initial array with SHUFFLE
    let first_row = records.next().unwrap()?;
    
    // Parse the array elements
    let mut shuffled_array: Vec<i32> = vec![];
    for data in first_row.iter().skip(1) {
        let num = data.parse::<i32>();
        shuffled_array.push(num.unwrap());
    }

    // 2. Prepare to store the remaining instructions
    let mut instructions: HashMap<Instruction, Vec<u16>> = HashMap::new();

    // 3. Iterate over the remaining rows
    for result in records {
        let record = result?;

        // Check if the record has at least one column for the instruction
        if record.is_empty() {
            continue; // Skip empty rows
        }

        // 4. Parse the instruction from the first column
        let instruction_str = &record[0]; // First column is the instruction
        let instruction: Instruction = instruction_str
            .parse()
            .map_err(|_| format!("Invalid instruction '{}'", instruction_str)).unwrap();

        // 5. Match based on the instruction
        match instruction {
            Instruction::SHUFFLE => {
                // For SHUFFLE, skip the instruction and parse the rest
                let mut values: Vec<u16> = vec![];
                for data in record.iter().skip(1) {
                    let value = data.parse::<u16>();
                    values.push(value.unwrap());
                }
                instructions.insert(instruction, values);
            }
            Instruction::SWAP => {
                // For SWAP, expect exactly 3 columns: SWAP, index1, index2
                let mut indices: Vec<u16> = vec![];
                for data in record.iter().skip(1) {
                    let index = data.parse::<u16>();
                    indices.push(index.unwrap());
                }
                instructions.insert(instruction, indices);
            }
            Instruction::STORE => {
                // For STORE, allow any number of additional columns after the instruction
                let mut values: Vec<u16> = vec![];
                for data in record.iter().skip(1) {
                    let value = data.parse::<u16>();
                    values.push(value.unwrap());
                }
                instructions.insert(instruction, values);
            }
        }
    }

    // 6. Return the initial array and instructions
    Ok((shuffled_array, instructions))
}
