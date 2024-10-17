mod sorting_algorithms;
mod utils;
mod display;
mod file_handling;

use display::{display, process_data};
use sorting_algorithms::*;
use utils::*;




fn main()
{
    let mut test_array: Vec<i32> = vec![5,2,1,6,3,4,9,1,8];
    
    for sort_func in SORT_FUNCTIONS.keys() {
        handle_sort(sort_func, &mut test_array);
    }

    process_data("selection_sort").unwrap();
}   
