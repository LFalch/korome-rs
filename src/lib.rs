#![warn(missing_docs, trivial_casts, trivial_numeric_casts)]
//! A small game engine written in Rust.

#[macro_use]
extern crate glium;
extern crate image;
#[macro_use]
extern crate quick_error;

mod draw;
mod logic;
mod vector;

/// Re-exports of crates
pub mod backend{
    /// Re-export of glium crate
    pub mod glium{
        pub use ::glium::*;
    }
}

pub use draw::{Graphics, Texture, Drawer, Draw};
pub use logic::{GameManager, Game, GameUpdate, Update, FrameInfo, VirtualKeyCode, MouseButton};
pub use vector::{Vector2, FloatVector};

/// Result type for `korome::TextureError`
pub type TextureResult = Result<Texture, TextureError>;
/// Result type for `glium::DrawError`
pub type DrawResult = Result<(), glium::DrawError>;

quick_error! {
    /// Wraps together all errors that can occur creating `Texture`s
    #[derive(Debug)]
    pub enum TextureError{
        /// A `glium::texture::TextureCreationError`
        TextureCreationError(err: glium::texture::TextureCreationError){
            from()
            cause(err)
            description("texture creation error")
        }
        /// An `image::ImageError`
        ImageError(err: image::ImageError){
            from()
            cause(err)
            description(err.description())
        }
        /// A `glium::vertex::buffer::CreationError`
        BufferCreationError(err: glium::vertex::BufferCreationError){
            from()
            cause(err)
            description(err.description())
        }
        /// An `std::io::Error`
        IoError(err: std::io::Error){
            from()
            cause(err)
            description(err.description())
        }
    }
}
