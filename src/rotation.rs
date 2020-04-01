#[derive(Debug, Copy, Clone)]
pub enum Rotation {
    Degree0,
    Degree90,
    Degree180,
    Degree270
}

impl Rotation {
    pub fn next(&self) -> Rotation {
        match self {
            Rotation::Degree0 => Rotation::Degree90,
            Rotation::Degree90 => Rotation::Degree180,
            Rotation::Degree180 => Rotation::Degree270,
            Rotation::Degree270 => Rotation::Degree0,
        }
    }
}