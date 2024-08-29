use bevy::prelude::*;

#[derive(Component)]
enum Dir {
    North, South, East, West
}

impl Dir {
    pub fn to_vec(&self) -> Vec3 {
        match self {
            Dir::North => Vec3::Z,
            Dir::South => Vec3::Z * -1.0,
            Dir::East => Vec3::X * -1.0,
            Dir::West => Vec3::X,
        }
    }

    pub fn all() -> Vec<Dir> {
        vec![Dir::North, Dir::East, Dir::South, Dir::West]
    }
}

#[derive(Component)]
struct Door {
    open: bool,
    position: Vec3,
}

#[derive(Component)]
pub struct Module {
    pub position: IVec2,
}

impl Module {
    pub fn new(x: i32, y: i32) -> Self {
        Module { position: IVec2::new(x, y) }
    }
}
