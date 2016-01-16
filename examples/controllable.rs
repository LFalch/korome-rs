#[macro_use]
extern crate korome;

use korome::*;

fn main() {
    // Create the draw object, which creates a window with the given title and dimensions
    let draw = Draw::new("korome works!", 800, 600);

    // Load a texture, whose bytes have been loaded at compile-time
    let planet = include_texture!(draw, "planet.png").unwrap();

    // Create a vector and push the objects to it
    let mut objs = Vec::new();
    objs.push(Object::new(&planet, -400., 300., 0.));

    // Create the game instance
    let mut game = Game::new(draw);

    while let Some((l_args, mut drawer)) = game.update() {
        logic(l_args, &mut objs);

        drawer.clear(0., 0., 1.);
        // Draw all sprites in objs
        drawer.draw_sprites(&objs).unwrap();
    }
}

fn logic(l_args: Update, objs: &mut Vec<Object>){
    // Get a mutable reference so we can move it
    let ref mut planet = objs[0];

    let delta = l_args.delta as f32;

    // Set the velocity to 200 pixels per second
    let vel = 200.0 * delta;
    let pos = &mut planet.pos;

    // Make the planet move with WASD and the arrow keys and rotate with Q and E
    is_down!{l_args;
        Left, A => {
            pos.0 -= vel
        },
        Right, D => {
            pos.0 += vel
        },
        Down , S => {
            pos.1 -= vel
        },
        Up   , W => {
            pos.1 += vel
        },
        Q => {
            planet.theta += delta
        },
        E => {
            planet.theta -= delta
        }
    }
}

struct Object<'a>{
    pos: Vector2<f32>,
    theta: f32,
    tex: &'a Texture
}

impl<'a> Object<'a>{
    fn new(tex: &'a Texture, x: f32, y: f32, theta: f32) -> Self{
        Object{
            tex: tex,
            pos: Vector2(x, y),
            theta: theta,
        }
    }
}

impl<'a> Sprite for Object<'a>{
    #[inline]
    fn get_pos(&self) -> (f32, f32){
        self.pos.into()
    }
    #[inline]
    fn get_rotation(&self) -> f32{
        self.theta
    }
    #[inline]
    fn get_texture(&self) -> &Texture{
        self.tex
    }
}
