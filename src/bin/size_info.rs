extern crate korome;
extern crate glium;

use korome::{Draw, Vector2, Game, Settings, LogicArgs, RenderArgs, GameLogic};
use korome::draw::{Texture, TextureDrawer};

use glium::texture::Texture2d;

use glium::backend::glutin_backend::GlutinFacade;

macro_rules! print_type_info{
    ($($t:ty),*) => {
        $({
            use std::mem::{align_of, size_of};

            let info = (size_of::<$t>(), align_of::<$t>());
            println!("{}\t: {} === {} * {}", stringify!($t), info.0, info.0/info.1, info.1);
        })*
    }
}

fn main(){
    println!("Version: {}", korome::VERSION);

    print_type_info!(Logic, Draw, Vector2, Game<Logic>, LogicArgs, RenderArgs, Settings, Texture2d, Texture, TextureDrawer, GlutinFacade);
}

struct Logic;

impl GameLogic for Logic {
    fn logic (&mut self, _: LogicArgs){

    }

    fn render(&self, _: RenderArgs){

    }
}
