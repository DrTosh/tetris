#[derive(Debug, Copy, Clone)]
pub enum Rotation {
    DEGREE_0,
    DEGREE_90,
    DEGREE_180,
    DEGREE_270
}

impl Rotation {
    pub fn next(&self) -> Rotation {
        match self {
            Rotation::DEGREE_0 => Rotation::DEGREE_90,
            Rotation::DEGREE_90 => Rotation::DEGREE_180,
            Rotation::DEGREE_180 => Rotation::DEGREE_270,
            Rotation::DEGREE_270 => Rotation::DEGREE_0,
        }
    }
}