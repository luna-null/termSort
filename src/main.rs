mod sorting_algorithms;
mod utils;
mod display;

use display::display;
use sorting_algorithms::*;
use utils::*;




fn main()
{
    let mut test_array: Vec<i32> = vec![5,2,1,6,3,4,9,1,8];
    
    for sort_func in SORT_FUNCTIONS.keys() {
        handle_sort(sort_func, &mut test_array);
    }

    display(&mut test_array);
}   
