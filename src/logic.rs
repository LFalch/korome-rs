extern crate glium;
extern crate time;

use std::collections::HashSet;

use super::{Draw, Drawer};
use time::precise_time_s as time_s;

use glium::{Surface};
use glium::glutin::{Event, ElementState};

pub use glium::glutin::VirtualKeyCode;

/// Manages a game loop
pub struct Game<'a>{
    draw  : Draw<'a>,
    last: f64,
    down_keys: HashSet<VirtualKeyCode>,
    mousepos: (i32, i32),
}

impl<'a> Game<'a>{
    #[inline]
    /// Creates a new `Game` from a `Draw`
    pub fn new(draw: Draw<'a>) -> Self {
        Game{
            draw: draw,
            last: time_s(),
            mousepos: (0, 0),
            down_keys: HashSet::new()
        }
    }

    /// Gets new updates and iniatiates drawing next frame
    pub fn update(&mut self) -> Option<(Update, Drawer)>{
        let mut keys = Vec::new();

        for ev in self.draw.poll_events() {
            match ev {
                Event::Closed => return None,
                Event::KeyboardInput(es, _, Some(vkc)) => match es{
                    ElementState::Pressed  => {
                        self.down_keys.insert( vkc);
                        keys.push((true , vkc));
                    },
                    ElementState::Released => {
                        self.down_keys.remove(&vkc);
                        keys.push((false, vkc));
                    }
                },
                Event::MouseMoved(pos) => self.mousepos = pos,
                _ => ()
            }
        }

        let now = time_s();
        let delta = now - self.last;
        self.last = now;

        let update = Update{
            delta    : delta,
            keyevents: keys,
            down_keys: &self.down_keys,
            mousepos : self.mousepos
        };

        Some((update, Drawer::new(&self.draw)))
    }
}

/// Wraps together useful data about what has happened (e.g. events)
#[derive(Debug)]
pub struct Update<'a>{
    /// The time that has passed since last update
    pub delta    : f64,
    /// The current position of the mouse
    pub mousepos : (i32, i32),
    /// A vector of all key events that have happened
    pub keyevents: Vec<(bool, VirtualKeyCode)>,

    /// A `HashSet` of all keys that are pressed down
    down_keys: &'a HashSet<VirtualKeyCode>
}

impl<'a> Update<'a>{
    /// Checks whether a key is pressed down
    pub fn is_down(&self, key: &VirtualKeyCode) -> bool{
        self.down_keys.contains(key)
    }
}

/// Macro for easily doing things if particular keys are down
/// # Example
///
/// ```rust
/// # macro_rules! is_down{($l_args:ident; $($($key:ident),+ => $b:block),+) => {}}
/// fn logic(player_y: &mut f32, l_args: korome::LogicArgs){
///     is_down!{l_args;
///         W, Up => {
///             player_y -= l_args.delta() as f32;
///         },
///         S, Down => {
///             player_y += l_args.delta() as f32;
///         }
///     };
/// }
/// ```
#[macro_export]
macro_rules! is_down{
    ( $l_args:ident; $( $( $key:ident ),+ => $b:block ),+ ) => {{
        $( if $( $l_args.is_down(&korome::VirtualKeyCode::$key) )||+ $b )+
    }}
}
