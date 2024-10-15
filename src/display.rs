use crossterm::{
    cursor, 
    event::{self, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers}, 
    execute, 
    terminal::{self, enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use std::{
    io::{self, stdout, Write}, 
    thread, 
    time::Duration
};

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


fn draw_bar(x_start: i16, y_start: i16, width: i16, height: i16) {
    for i in 0..height {
        execute!(stdout(), cursor::MoveTo(x_start as u16, (y_start - i) as u16)).unwrap();
        print!("{}", "#".repeat(width as usize));
        stdout().flush().unwrap();
    }
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

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), cursor::Hide)?;
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

    for (index, value) in array.iter().enumerate() {
        draw_bar(
            width_bar_begin + (index as i16) * bar_width,
            height_bar_end as i16,
            bar_width as i16,
            *value as i16 * V_SCALING,
        );
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
                    _ => {
                        println!("You pressed a key!");
                    },
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
