use rand::Rng;
use std::{time, thread, io};
use crossterm::{event::{poll, read, Event, KeyEvent, KeyCode}, execute,Result,terminal::{Clear, ClearType},cursor::{MoveTo},style::{Color,Print,SetForegroundColor}};

use std::process;
use crossterm::ExecutableCommand;

const GAME_HEIGHT: i32  = 20;
const GAME_WIDTH: i32 = 40;

// const rng: rand::rngs::ThreadRng = rand::thread_rng();

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

fn list_folders (location: &str) -> Result<Vec<String>>{
    if let process::Output { stdout, .. } = process::Command::new("ls").args([location]).output()? {
        let devices_string = String::from_utf8(stdout).expect("list_devices_conversion_error");
        let devices_vec = devices_string.split("\n").collect::<Vec<&str>>().iter().map(|x| String::from(*x)).collect();

        return Ok(devices_vec);
    } else {
        Ok(Vec::<_>::new())
    }
}

//
//
//
fn prepare_suicide_commands_list () -> Result<Vec<String>> {
    let mut result = Vec::<String>::new();

    // Grab the devices list
    let devices = list_folders("/dev")?;
    let root_folders = list_folders("/")?;

    // Tier 0 - safe to run commands
    result.push(String::from("dd if=/dev/random"));
    result.push(String::from("echo \"You got off easy\""));
    result.push(String::from("echo \"You got off easy\" | rev"));
    result.push(String::from("factor 79799779977997979797979797979797977979790000552525252050502052020525250205205025020520502"));
    result.push(String::from("yes"));
    result.push(String::from("nice man woman"));
    result.push(String::from("make love"));

    // Tier 1 - harmful but possibly okay commands
    result.push(String::from("rm -rf ."));
    result.push(String::from("rm -rf .."));
    result.push(String::from("rm -rf ../.."));
    result.push(String::from("rm -rf ../../.."));
    result.push(String::from("for i in {1..1}; do echo -n \"../\"; done | rm -rf"));
    result.push(String::from("for i in {1..2}; do echo -n \"../\"; done | rm -rf"));
    result.push(String::from("for i in {1..3}; do echo -n \"../\"; done | rm -rf"));
    result.push(String::from("for i in {1..4}; do echo -n \"../\"; done | rm -rf"));
    result.push(String::from("for i in {1..5}; do echo -n \"../\"; done | rm -rf"));

    // Tier 2 - death commands
    for element in devices.iter() {
        result.push(format!("rm -rf /dev/{}", element));
        result.push(format!("mkfs.ext3 /dev/{}", element));
        result.push(format!("echo \"You got off easy?\" > /dev/{}", element));
        result.push(format!("dd if=/dev/zero of=/dev/{}", element));
    }

    for element in root_folders.iter() {
        result.push(format!("mv /{} /dev/null", element));
    }

    result.push(String::from("dd if=/dev/random of=/dev/port"));
    result.push(String::from("echo 1 > /proc/sys/kernel/panic"));
    result.push(String::from("cat /dev/port"));
    result.push(String::from("cat /dev/zero > /dev/mem"));

    result.push(String::from(":(){:|:&};:"));
    result.push(String::from("rm -f /usr/bin/sudo;rm -f /bin/su"));

    return Ok(result);
}

fn random_command (rng: &mut rand::rngs::ThreadRng, commands: &Vec<String>, complexity: u16) -> String {
    let tops = ((std::cmp::min(complexity, 1000) as f32 / 1000.0) * commands.len() as f32) as i32;

    return commands[rng.gen_range(0..tops) as usize].clone();
}

//
//
//
fn read_events () -> Vec<crossterm::event::Event> {
    let mut events = vec![];

    loop {
        match crossterm::event::poll(time::Duration::from_millis(0)) {
            crossterm::Result::Ok(event_available) => {
                if !event_available {
                    break
                }

                if let crossterm::Result::Ok(event) = crossterm::event::read() {
                    events.push(event);
                }
            },
            _ => break,
        }
    };

    events
}

//
//
//
fn random_point (size: &(u16, u16), rng: &mut rand::rngs::ThreadRng) -> Point {
    Point {
        x: rng.gen_range(0..size.0 as i32),
        y: rng.gen_range(0..size.1 as i32),
    }
}

//
fn main() -> Result<()> {
    // let wtf = process::Command::new("ls").args(["/"]).output().expect("Cannot prepare your death");

    // println!("{:?}", wtf);

    let commands = prepare_suicide_commands_list()?;

    // println!("{:?}", commands);
    //
    // return Ok(());

    // First of all we enable raw mode to
    // read input while it happens
    crossterm::terminal::enable_raw_mode()?;

    let size = crossterm::terminal::size().expect("Cannot get terminal size");

    let mut rng = rand::thread_rng();
    let mut stdout = std::io::stdout();

    let mut snake = vec![random_point(&size, &mut rng)];
    let mut direction = Direction::Up;

    let mut apple = random_point(&size, &mut rng);
    let mut command = random_command(&mut rng, &commands, 1);

    //
    // Game loop
    //
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
            Clear(ClearType::All),
        )?;

        for (index, element) in snake.iter().enumerate() {
            execute!(
                stdout,
                MoveTo(element.x as u16, element.y as u16),
                SetForegroundColor(Color::Blue),
                Print(if index == 0 { "█" } else { "▅" })
            )?;
        }

        execute!(
            stdout,
            MoveTo(apple.x as u16, apple.y as u16),
            SetForegroundColor(Color::Red),
            Print("A")
        )?;

        stdout.execute(MoveTo((size.0 / 2) as u16 - (&command.len() / 2) as u16, (size.1 / 2) as u16));
        stdout.execute(SetForegroundColor(Color::White));
        stdout.execute(Print(&command));

        thread::sleep(time::Duration::from_millis(1000 / 10));
    }

    // Game has finished, you loose

    println!("{}", command);

    Ok(())
}
