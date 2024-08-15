#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

pub const PALETTE: [Color; 0x40] = [
   Color(0x70, 0x70, 0x70), Color(0x00, 0x00, 0xA8), Color(0x20, 0x18, 0x88), Color(0x40, 0x00, 0x98),
   Color(0x88, 0x00, 0x70), Color(0xA8, 0x00, 0x10), Color(0xA0, 0x00, 0x00), Color(0x78, 0x08, 0x00),
   Color(0x40, 0x28, 0x00), Color(0x00, 0x40, 0x00), Color(0x00, 0x50, 0x00), Color(0x00, 0x38, 0x10),
   Color(0x18, 0x38, 0x58), Color(0x0E, 0x0E, 0x16), Color(0x0E, 0x0E, 0x16), Color(0x0E, 0x0E, 0x16),
   Color(0xB8, 0xB8, 0xB8), Color(0x00, 0x70, 0xE8), Color(0x20, 0x38, 0xE8), Color(0x80, 0x00, 0xF0),
   Color(0xB8, 0x00, 0xB8), Color(0xE0, 0x00, 0x58), Color(0xD8, 0x28, 0x00), Color(0xC8, 0x48, 0x08),
   Color(0x88, 0x70, 0x00), Color(0x00, 0x90, 0x00), Color(0x00, 0xA8, 0x00), Color(0x00, 0x90, 0x38),
   Color(0x00, 0x80, 0x88), Color(0x0E, 0x0E, 0x16), Color(0x0E, 0x0E, 0x16), Color(0x0E, 0x0E, 0x16),
   Color(0xF8, 0xF8, 0xF8), Color(0x38, 0xB8, 0xF8), Color(0x93, 0xb4, 0xFa), Color(0xA0, 0x88, 0xF8),
   Color(0xF0, 0x78, 0xF8), Color(0xF8, 0x70, 0xB0), Color(0xF8, 0x70, 0x60), Color(0xF8, 0x98, 0x38),
   Color(0xF0, 0xB8, 0x38), Color(0x80, 0xD0, 0x10), Color(0x48, 0xD8, 0x48), Color(0x58, 0xF8, 0x98),
   Color(0x00, 0xE8, 0xD8), Color(0x50, 0x50, 0x50), Color(0x0E, 0x0E, 0x16), Color(0x0E, 0x0E, 0x16),
   Color(0xF8, 0xF8, 0xF8), Color(0xA8, 0xE0, 0xF8), Color(0xC0, 0xD0, 0xF8), Color(0xD0, 0xC8, 0xF8),
   Color(0xF8, 0xC0, 0xF8), Color(0xF8, 0xC0, 0xD8), Color(0xF8, 0xB8, 0xB0), Color(0xF8, 0xD8, 0xA8),
   Color(0xF8, 0xE0, 0xA0), Color(0xE0, 0xF8, 0xA0), Color(0xA8, 0xF0, 0xB8), Color(0xB0, 0xF8, 0xC8),
   Color(0x98, 0xF8, 0xF0), Color(0x98, 0x98, 0x98), Color(0x0E, 0x0E, 0x16), Color(0x0E, 0x0E, 0x16)
];
