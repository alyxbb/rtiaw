pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const UNIVERSE: Self = Self {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn countains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surronds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}
