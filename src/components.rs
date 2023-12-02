use std::time::Duration;

use bevy::prelude::*;


#[derive(Component)]
pub struct Clock {
    timer: Timer
}


impl Clock {
    pub fn seconds(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating)
        }
    }

    pub fn seconds_once(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Once)
        }
    }

    pub fn millis(millis: u64) -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(millis), TimerMode::Repeating)
        }
    }

    #[allow(dead_code)]
    pub fn millis_once(millis: u64) -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(millis), TimerMode::Once)
        }
    }

    pub fn tick(&mut self, duration: Duration) {
        self.timer.tick(duration);
    }

    pub fn finished(&self) -> bool {
        self.timer.finished()
    }

    pub fn just_finished(&self) -> bool {
        self.timer.just_finished()
    }
}
