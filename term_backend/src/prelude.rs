// Re-export crossterm structs for easier access
pub use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
pub use crossterm::style::{Attribute, Attributes};

pub use crate::{CrosstermPlugin, CrosstermWindow, CrosstermWindowSettings, Cursor};
pub use crate::components::{
    Color, Colors, Position, Sprite, SpriteBundle, Style, StyleMap, Visible,
};

