use std::io::{Write, stdout};
use crossterm::{cursor, event::{Event, KeyCode, read}, execute, ExecutableCommand, terminal};

fn main() {
    // Clear the screen
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    // Write instructions
    println!("Use the arrow keys to move the cursor by zero. It should not do anything once crossterm has been fixed.");
    println!("Press escape to exit.");

    // Use raw mode so the cursor can be moved
    terminal::enable_raw_mode().unwrap();

    // Keep reading input until escape is pressed
    loop {
        let event = read().unwrap();

        match event {
            Event::Key(key) => {
                match key.code {
                    // Moving by zero actually moves by one in crossterm 0.17.7
                    KeyCode::Left => execute!(stdout, cursor::MoveLeft(0)).unwrap(),
                    KeyCode::Right => execute!(stdout, cursor::MoveRight(0)).unwrap(),
                    KeyCode::Up => execute!(stdout, cursor::MoveUp(0)).unwrap(),
                    KeyCode::Down => execute!(stdout, cursor::MoveDown(0)).unwrap(),
                    KeyCode::Esc => {
                        // Leave the terminal in a clean state
                        terminal::disable_raw_mode().unwrap();
                        stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
                        break;
                    }
                    _ => {},
                };
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }
}
