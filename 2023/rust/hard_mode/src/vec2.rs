#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vec2(pub i64, pub i64);

impl Vec2 {
    pub fn add(&self, direction: &str, distance: i64) -> Option<Self> {
        Some(match direction {
            "U" | "3" => Self(self.0 + distance, self.1),
            "D" | "1" => Self(self.0 - distance, self.1),
            "L" | "2" => Self(self.0, self.1 - distance),
            "R" | "0" => Self(self.0, self.1 + distance),
            _ => return None,
        })
    }
}
