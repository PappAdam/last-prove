use std::fmt::Debug;

use super::Map;

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.matrix {
            for tile in row {
                if tile.solid {
                    write!(f, "X").ok();
                } else {
                    write!(f, " ").ok();
                }
            }
            writeln!(f, "").ok();
        }
        Ok(())
    }
}
