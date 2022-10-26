use rand::Rng;
use crossterm;

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

pub fn read_events () -> Vec<crossterm::event::Event> {
    let mut events = vec![];

    loop {
        match crossterm::event::poll(std::time::Duration::from_millis(0)) {
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

pub fn random_point (size: &(u16, u16), rng: &mut rand::rngs::ThreadRng) -> Point {
    Point {
        x: rng.gen_range(0..size.0 as i32),
        y: rng.gen_range(0..size.1 as i32),
    }
}
