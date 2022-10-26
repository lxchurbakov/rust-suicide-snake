use std::io;
use crossterm::{ terminal, cursor, style };
use crate::gameutils::read_events;

pub fn print_intro (stdout: &mut io::Stdout) -> crossterm::Result<()> {
    let size = crossterm::terminal::size()?;
    let midscreen = (size.0 / 2) as u16;

    crossterm::execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(midscreen - 12, 5),
        style::SetForegroundColor(style::Color::White),
        style::Print("Welcome to suicide snake!")
    )?;

    crossterm::execute!(
        stdout,
        style::SetForegroundColor(style::Color::White),

        cursor::MoveTo(midscreen - 29, 5),
        style::Print("It's a classic snake game, but in the center of the screen"),
        cursor::MoveTo(midscreen - 34, 6),
        style::Print("you will see a command. Once you loose, the command will be executed"),
        cursor::MoveTo(midscreen - 9, 8),
        style::Print("Hit enter to start"),
    )?;

    'intro: loop {
        for event in read_events().iter() {
            match event {
                crossterm::event::Event::Key(key_event) => {
                    // And finally we check if it's a KeyEvent
                    let crossterm::event::KeyEvent { code, .. } = key_event;

                    if let crossterm::event::KeyCode::Enter = code {
                        break 'intro;
                    }
                },
                _ => (),
            }
        }
    }

    Ok(())
}
