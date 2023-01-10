#[derive(Debug, Clone, PartialEq)]
pub enum MouseButtonState {
    None,
    Left,
    Right,
    Both,
}

impl Default for MouseButtonState {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyboardState {
    Up,
    Left,
    Right,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self::Up
    }
}
