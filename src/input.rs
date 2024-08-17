use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Component)]
pub struct InputAxis {
    pub neg: Vec<KeyCode>,
    pub pos: Vec<KeyCode>,
}

#[derive(Resource)]
pub struct InputAxes {
    axes: HashMap<String, InputAxis>,
}

impl Default for InputAxes {
    fn default() -> Self {
        let mut axes = HashMap::new();
        axes.insert(
            String::from("Vertical"),
            InputAxis {
                neg: vec![KeyCode::ArrowDown, KeyCode::KeyS],
                pos: vec![KeyCode::ArrowUp, KeyCode::KeyW],
            },
        );
        axes.insert(
            String::from("Horizontal"),
            InputAxis {
                neg: vec![KeyCode::ArrowRight, KeyCode::KeyD],
                pos: vec![KeyCode::ArrowLeft, KeyCode::KeyA],
            },
        );

        Self { axes }
    }
}

impl InputAxes {
    pub fn get(&self, name: &str) -> &InputAxis {
        &self.axes[name]
    }
}
