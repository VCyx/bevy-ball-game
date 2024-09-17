use bevy::prelude::{Resource, Timer, TimerMode};

const STAR_SPAWN_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating)
        }
    }
}
