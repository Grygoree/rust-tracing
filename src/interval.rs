#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32
}

pub const EMPTY: Interval = Interval::new(f32::INFINITY, f32::NEG_INFINITY);
pub const UNIVERSE: Interval = Interval::new(f32::NEG_INFINITY, f32::INFINITY);

impl Interval {
    pub const fn new (min: f32, max : f32) -> Interval { Interval {min, max}}
    pub fn contains(self, x: f32) -> bool { x >= self.min && x <= self.max }
    pub fn surrounds( self, x: f32) -> bool { x > self.min && x < self.max }
    pub fn clamp(self, x: f32) -> f32 {
        if x < self.min { return self.min }
        if x > self.max { return self.max }
        x
    }
}

impl Default for Interval {
    fn default() -> Self {
        EMPTY
    }
}
