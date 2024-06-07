extern crate alloc;

use crate::coordinate::PolarCoordinate;
use crate::common::MotionFrame;
use alloc::collections::VecDeque;
use libm::ceil;

struct Checkpoint {
    position: PolarCoordinate,
    vector: PolarCoordinate,
    absolute_distance: f64,
    steps: i16,
    step_size: f64,
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
        let last_checkpoint = &self.checkpoints.back();
        let prev_position = match last_checkpoint {
            None => &self.config.home_position,
            checkpoint_opt => &checkpoint_opt.unwrap().position,
        };

        if prev_position.equals(&position) {
            return;
        }

        let prev_absolute_distance = match last_checkpoint {
            None => 0.0,
            checkpoint_opt => checkpoint_opt.unwrap().absolute_distance,
        };
        let distance = prev_position.distance(&position);
        let steps = ceil(distance / self.config.step_distance) as i16;

        self.checkpoints.push_back(Checkpoint {
            position: position.clone(),
            vector: position.subtract(&prev_position),
            absolute_distance: distance + prev_absolute_distance,
            steps,
            step_size: distance / (steps as f64),
        });
    }

    pub fn is_queue_ready(&self) -> bool {
        let last_absolute_position = match self.checkpoints.back() {
            None => 0.0,
            _ => self.checkpoints.back().unwrap().absolute_distance,
        };
        last_absolute_position - self.next_slowdown_distance() < 0.5
    }

    pub fn next_frame(&mut self) -> MotionFrame {
        let has_current_frame = match &self.current_frame {
            None => false,
            _ => true,
        };

        if !has_current_frame {
            self.current_frame = Some(MotionFrame {
                position: self.config.home_position.clone(),
                speed: 0.0,
                relative_distance: 0.0,
                absolute_distance: 0.0,
            });
        }

        if self.checkpoints.len() == 0 {
            return self.current_frame.as_ref().unwrap().clone()
        }

        let current_checkpoint = self.checkpoints.front().unwrap();
        let current_frame = self.current_frame.as_ref().unwrap();
        let next_slowdown_distance = self.next_slowdown_distance();
        let acceleration_direction = match next_slowdown_distance > current_frame.absolute_distance {
            true => 1.0,
            false => -1.0,
        };
        let step_size = current_checkpoint.step_size;
        let steps = current_checkpoint.steps;
        let slowdown_relative_distance = next_slowdown_distance - current_frame.absolute_distance;
        let speed_delta = match slowdown_relative_distance > 0.0
            && slowdown_relative_distance < current_checkpoint.step_size {
            true => 0.0,
            false => self.config.max_acceleration * acceleration_direction * step_size,
        };
        let slowdown_relative_distance = next_slowdown_distance - current_frame.absolute_distance;
        let mut speed = current_frame.speed + (self.config.max_acceleration * acceleration_direction * step_size);
        if slowdown_relative_distance > 0.0 && slowdown_relative_distance < current_checkpoint.step_size {
            // hold speed as we don't have enough runway to accelerate, and it's too early to decelerate
            speed = current_frame.speed;
        }
        let scaled_vector = current_checkpoint.vector.scale(1.0 / (steps as f64));
        let next_frame = MotionFrame {
            position: current_frame.position.add(&scaled_vector),
            speed: speed.min(self.config.max_speed).max(self.config.min_speed),
            relative_distance: step_size,
            absolute_distance:  current_frame.absolute_distance + step_size,
        };

        if next_frame.absolute_distance >= current_checkpoint.absolute_distance {
            self.checkpoints.pop_front();
        }

        self.current_frame = Some(next_frame.clone());
        next_frame
    }

    fn next_stop_distance(&self) -> f64 {
        let start_checkpoint: &Checkpoint = &self.checkpoints.front().unwrap();
        let start_direction = start_checkpoint.vector.direction();
        let mut absolute_distance = start_checkpoint.absolute_distance;
        for checkpoint_index in 0..self.checkpoints.len() {
            let current_checkpoint = &self.checkpoints[checkpoint_index];
            let has_direction_changed = !start_direction.equals(&current_checkpoint.vector.direction());
            if has_direction_changed {
                return absolute_distance;
            }
            absolute_distance = current_checkpoint.absolute_distance;
        }

        self.checkpoints.back().unwrap().absolute_distance
    }

    fn next_slowdown_distance(&self) -> f64 {
        let current_speed = match &self.current_frame {
            None => 0.0,
            frame => frame.as_ref().unwrap().speed
        };
        let steps_to_stop = current_speed / self.config.max_acceleration;
        self.next_stop_distance() - (steps_to_stop * self.config.step_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speedup_slowdown() {
        let home_position = PolarCoordinate { theta: 0.0, rho: 0.0 };
        let mut motion_controller = MotionController::new(MotionControllerConfig {
            home_position: home_position.clone(),
            max_acceleration: 1.0,
            max_speed: 100.0,
            min_speed: 1.0,
            step_distance: 0.1,
        });
        motion_controller.queue_position(PolarCoordinate { theta: 0.0, rho: 0.3 });
        assert_eq!(motion_controller.is_queue_ready(), true);
        motion_controller.queue_position(PolarCoordinate { theta: -0.4, rho: 0.4 });
        assert_eq!(motion_controller.is_queue_ready(), true);
        motion_controller.queue_position(PolarCoordinate { theta: 0.0, rho: 0.0 });
        assert_eq!(motion_controller.is_queue_ready(), false);
        let frame0 = motion_controller.next_frame();
        assert_eq!(frame0.speed, 1.0);
        assert_eq!(frame0.position.theta, 0.0);
        assert_eq!(frame0.position.rho, 0.09999999999999999);
        let frame1 = motion_controller.next_frame();
        assert_eq!(frame1.speed, 1.1);
        assert_eq!(frame1.position.theta, 0.0);
        assert_eq!(frame1.position.rho, 0.19999999999999998);
        let frame2 = motion_controller.next_frame();
        assert_eq!(frame2.speed, 1.0);
        assert_eq!(frame2.position.theta, 0.0);
        assert_eq!(frame2.position.rho, 0.3);
        let frame3 = motion_controller.next_frame();
        assert_eq!(frame3.speed, 1.08);
        assert_eq!(frame3.position.theta, -0.13333333333333333);
        assert_eq!(frame3.position.rho, 0.3333333333333333);
        let frame4 = motion_controller.next_frame();
        assert_eq!(frame4.speed, 1.08);
        assert_eq!(frame4.position.theta, -0.26666666666666666);
        assert_eq!(frame4.position.rho, 0.36666666666666664);
        let frame5 = motion_controller.next_frame();
        assert_eq!(frame5.speed, 1.0);
        assert_eq!(frame5.position.theta, -0.4);
        assert_eq!(frame5.position.rho, 0.39999999999999997);
        let frame6 = motion_controller.next_frame();
        assert_eq!(frame6.speed, 1.096);
        assert_eq!(frame6.position.theta, -0.32);
        assert_eq!(frame6.position.rho, 0.31999999999999995);
        let frame7 = motion_controller.next_frame();
        assert_eq!(frame7.speed, 1.1920000000000002);
        assert_eq!(frame7.position.theta, -0.24);
        assert_eq!(frame7.position.rho, 0.23999999999999994);
        let frame8 = motion_controller.next_frame();
        assert_eq!(frame8.speed, 1.2880000000000003);
        assert_eq!(frame8.position.theta, -0.15999999999999998);
        assert_eq!(frame8.position.rho, 0.15999999999999992);
        let frame9 = motion_controller.next_frame();
        assert_eq!(frame9.speed, 1.2880000000000003);
        assert_eq!(frame9.position.theta, -0.07999999999999996);
        assert_eq!(frame9.position.rho, 0.0799999999999999);
    }
}
