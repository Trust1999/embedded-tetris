mod ma72xx;
pub use ma72xx::Max72xx;

mod text;
pub use text::TextDisplay;

pub mod render;

pub trait Display: Sized {
    fn fill(&mut self, value: bool);

    fn set_pixel(&mut self, x: u8, y: u8, value: bool);
}
