use std::collections::VecDeque;
use crate::coordinate::PolarCoordinate;

struct Checkpoint {
    position: PolarCoordinate,
    vector: PolarCoordinate,
    cumulative_distance: f64,
}

pub struct MotionFrame {
    pub position: PolarCoordinate,
    pub speed: f64,
    pub travel_distance: f64,
    cumulative_distance: f64,
}

impl MotionFrame {
    fn copy(&self) -> Self {
        MotionFrame {
            position: self.position.copy(),
            speed: self.speed,
            travel_distance: self.travel_distance,
            cumulative_distance: self.cumulative_distance,
        }
    }
}

pub struct MotionControllerConfig {
    pub home_position: PolarCoordinate,
    pub max_acceleration: f64,
    pub max_speed: f64,
    pub min_speed: f64,
    pub step_distance: f64,
}

pub struct MotionController {
    checkpoints: VecDeque<Checkpoint>,
    current_frame: Option<MotionFrame>,
    config: MotionControllerConfig,
}

impl MotionController {
    pub fn new(config: MotionControllerConfig) -> Self {
        MotionController {
            checkpoints: VecDeque::new(),
            current_frame: None,
            config,
        }
    }
    pub fn queue_position(&mut self, position: PolarCoordinate) {
        let prev_position: &PolarCoordinate = match &self.checkpoints.back() {
            None => &self.config.home_position,
            checkpoint_opt => &checkpoint_opt.unwrap().position,
        };

        if prev_position.equals(&position) {
            return;
        }

        self.checkpoints.push_back(Checkpoint {
            position: position.copy(),
            vector: prev_position.vector_to(&position),
            cumulative_distance: prev_position.distance(&position),
        });
    }

    pub fn is_queue_ready(&self) -> bool {
        let checkpoints_present = match self.checkpoints.back() {
            None => false,
            _ => true,
        };

        if !checkpoints_present {
            return true;
        }

        self.distance_at_stop() + self.config.step_distance < self.checkpoints.back().unwrap().cumulative_distance
    }

    pub fn next_frame(&mut self) -> MotionFrame {
        let MotionControllerConfig {
            home_position,
            max_acceleration,
            max_speed,
            min_speed,
            step_distance
        } = &self.config;
        let has_current_frame = match &self.current_frame {
            None => false,
            _ => true,
        };

        if !has_current_frame {
            let next_frame = MotionFrame {
                position: home_position.copy(),
                speed: *min_speed,
                travel_distance: 0.0,
                cumulative_distance: 0.0,
            };
            self.current_frame = Some(next_frame.copy());
            return next_frame;
        }

        if self.checkpoints.len() == 0 {
           return self.current_frame.as_ref().unwrap().copy()
        }

        let current_checkpoint = *&self.checkpoints.front().unwrap();
        let current_frame = self.current_frame.as_ref().unwrap();
        let scaled_vector = &current_checkpoint.vector.scale(*step_distance);
        let next_position = PolarCoordinate {
            theta: current_frame.position.theta + scaled_vector.theta,
            rho: current_frame.position.rho + scaled_vector.rho,
        };

        let travel_distance = current_frame.position.distance(&next_position);
        let cumulative_distance = current_frame.cumulative_distance + travel_distance;
        let steps_to_stop = (self.next_stop_distance() - cumulative_distance) / step_distance;
        let acceleration_direction = match steps_to_stop > current_frame.speed * max_acceleration {
            true => 1.0,
            false => -1.0,
        };
        let speed = current_frame.speed + (max_acceleration * acceleration_direction);
        let next_frame = MotionFrame {
            position: next_position.copy(),
            speed: speed.min(*max_speed).max(*min_speed),
            travel_distance,
            cumulative_distance,
        };

        self.current_frame = Some(next_frame.copy());

        if next_frame.cumulative_distance >= current_checkpoint.cumulative_distance {
            self.checkpoints.pop_front();
        }

        next_frame
    }

    fn next_stop_distance(&self) -> f64 {
        let lookahead_distance = self.distance_at_stop();
        let start_checkpoint: &Checkpoint = &self.checkpoints.front().unwrap();
        let start_direction = start_checkpoint.vector.direction();
        for checkpoint_index in 0..self.checkpoints.len() {
            let current_checkpoint = &self.checkpoints[checkpoint_index];
            let cumulative_distance = current_checkpoint.cumulative_distance;
            let has_direction_changed = !start_direction.equals(&current_checkpoint.vector.direction());
            if cumulative_distance > lookahead_distance || has_direction_changed {
                return cumulative_distance;
            }
        }

        self.checkpoints.back().unwrap().cumulative_distance
    }

    fn distance_at_stop(&self) -> f64 {
        let frame_present = match &self.current_frame {
            None => false,
            _ => true,
        };

        if !frame_present {
            return 0.0;
        }
        let current_frame = self.current_frame.as_ref().unwrap();
        let relative_steps_to_stop = current_frame.speed / self.config.max_acceleration;
        current_frame.cumulative_distance + (relative_steps_to_stop / self.config.step_distance)
    }
}

