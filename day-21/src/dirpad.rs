use glam::{ivec2, IVec2};

pub struct DirPad {
    keypad: [(super::Dir, IVec2); 5],
}

impl Default for DirPad {
    fn default() -> Self {
        use super::Dir::*;

        #[rustfmt::skip]
        let keypad = [
                                 (Up,   ivec2(1, 0)), (Action, ivec2(2, 0)),
            (Left, ivec2(0, 1)), (Down, ivec2(1, 1)), (Right,  ivec2(2, 1)),
        ];

        Self { keypad }
    }
}
