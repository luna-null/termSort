use crossterm::{
    cursor, 
    event::{self, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers}, 
    execute, 
    terminal::{self, enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use std::{
    error::Error, fs::File, io::{self, stdout, Write}, thread, time::Duration
};

use crate::file_handling::{open_read_data_file, read_from_data};

/* TODO:
 * - store initial shuffled array at beginning of sort
 * - make swap display function
 * - make insert display function (and regular function)
 * - make stored display function
 * - errors
 * - tests
 * - headless version
 * - colors for bars
 * - control panel at bottom
 * - outsource file opening and writing so it doesnt happen during sort, and sorts return vector of vectors of strings
 */

const MARGIN_H: i16 = 3;
const MARGIN_V: i16 = 1;
const V_SCALING: i16 = 3;

struct Bar
{
    x_start: i16,
    y_start: i16,
    width: i16,
    height: i16,
}


impl Bar {
    fn draw_bar(x_start: i16, y_start: i16, width: i16, height: i16) -> Result<Bar, Box<dyn Error>> {
        for i in 0..height {
            // Move cursor to position and print the bar
            execute!(stdout(), cursor::MoveTo(x_start as u16, (y_start - i) as u16))?;
            print!("{}", "#".repeat(width as usize));
        }

        stdout().flush()?;  // Make sure to flush stdout
        // Return the Bar struct after drawing it
        Ok(Bar {
            x_start,
            y_start,
            width,
            height,
        })
    }
    fn swap_bar_pos<'a,'b>(bar1: &'a mut Bar, bar2: &'b mut Bar) -> (&'b mut Bar, &'a mut Bar)
    {
        std::mem::swap(&mut bar1.x_start, &mut bar2.x_start); 
        std::mem::swap(&mut bar1.y_start, &mut bar2.y_start); 
        (bar2, bar1)
    }
}


pub fn process_data(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file: File = open_read_data_file(filename)?;
    let (mut shuffled_array, instructions) = read_from_data(file).unwrap();

    display(&mut shuffled_array)?;

    Ok(())

}

pub fn display(array: &mut Vec<i32>) -> io::Result<()> {

    let bar_length = array.len() as u16;
    let (term_width, term_height) = terminal::size().unwrap();
    let width_bar_begin = MARGIN_H;
    let width_bar_end = term_width as i16 - MARGIN_H;
    let _height_bar_begin = MARGIN_V;
    let height_bar_end = term_height as i16 - MARGIN_V;
    let bar_draw_space_h = width_bar_end - width_bar_begin;
    let bar_width = bar_draw_space_h / bar_length as i16;

    let mut bar_array: Vec<Bar> = vec![];
    let mut bar: Bar;   
    
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), cursor::Hide)?;
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

    for (index, value) in array.iter().enumerate() {
        bar = Bar::draw_bar(
            width_bar_begin + (index as i16) * bar_width,
            height_bar_end as i16,
            bar_width as i16,
            *value as i16 * V_SCALING,
        ).unwrap();
        bar_array.push(bar);

    }
    stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(50)); // Adjust refresh rate if needed

    // Handle user input in the main thread
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(event) = event::read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::NONE,
                        kind: KeyEventKind::Press,
                        state: KeyEventState::NONE,
                    } => {
                        println!("Exiting...");
                        break;
                    },
                    _ => {},
                }
            }
        }
    }

    // Cleanup before exiting
    execute!(stdout(), cursor::Show)?;
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
