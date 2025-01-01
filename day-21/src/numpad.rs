use glam::{ivec2, IVec2};

pub const A: u8 = u8::MAX;

pub struct NumPad {
    pub keypad: [(char, IVec2); 11],
    pub action: IVec2,
}

impl Default for NumPad {
    fn default() -> Self {
        let action = ivec2(2, 3);

        #[rustfmt::skip]
        let keypad = [
            ('7', ivec2(0, 0)), ('8', ivec2(1, 0)), ('9', ivec2(2, 0)),
            ('4', ivec2(0, 1)), ('5', ivec2(1, 1)), ('6', ivec2(2, 1)),
            ('1', ivec2(0, 2)), ('2', ivec2(1, 2)), ('3', ivec2(2, 2)),
                                ('0', ivec2(1, 3)), ('A', action),
        ];

        Self { keypad, action }
    }
}
