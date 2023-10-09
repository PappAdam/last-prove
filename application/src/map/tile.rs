#[derive(Clone)]
pub struct Tile {
    flags: u8,
    //0: Solid
    //1: Building on top
    //2: Troop on top
    //3: NOT USED
    //4: NOT USED
    //5: NOT USED
    //6: NOT USED
    //7: NOT USED
}

impl Tile {
    pub fn is_solid(&self) -> bool {
        self.flags & TileFlag::Solid as u8 == TileFlag::Solid as u8
    }
    pub fn flag_active(&self, flag: TileFlag) -> bool {
        self.flags & flag as u8 == flag as u8
    }
    pub fn set_flag(&mut self, flag: TileFlag) {
        self.flags |= flag as u8
    }
    pub(super) fn new() -> Self {
        Self {
            flags: TileFlag::Solid as u8,
        }
    }
    pub(super) fn none() -> Self {
        Self { flags: 0 }
    }
}

#[derive(Clone, Copy)]
pub enum TileFlag {
    Solid = 0b10000000,
    BuildingOnTop = 0b01000000,
    TroopOnTop = 0b00100000,
}
