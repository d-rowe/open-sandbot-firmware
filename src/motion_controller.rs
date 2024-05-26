use crate::coordinate::PolarCoordinate;

const HOME_POSITION: PolarCoordinate = PolarCoordinate { theta: 0.0, rho: 0.0 };
const MAX_ACCELERATION: f64 = 0.5;
const MAX_SPEED: f64 = 100.0;
const STEP_DISTANCE: f64 = 0.5;

struct Checkpoint {
    index: usize,
    position: PolarCoordinate,
    vector: PolarCoordinate,
    total_distance: f64,
}

impl Checkpoint {
    fn is_direction_change(&self, other: &Checkpoint) -> bool {
        !self.vector.direction().equals(&other.vector.direction())
    }
}

pub struct MotionFrame {
    position: PolarCoordinate,
    speed: f64,
    total_distance: f64,
}

impl MotionFrame {
    fn copy(&self) -> Self {
        MotionFrame {
            position: self.position.copy(),
            speed: self.speed,
            total_distance: self.total_distance,
        }
    }
}

pub struct MotionController {
    current_checkpoint_index: usize,
    checkpoints: Vec<Checkpoint>,
    current_frame: Option<MotionFrame>,
}

impl MotionController {
    pub fn new() -> Self {
        MotionController {
            current_checkpoint_index: 0,
            checkpoints: Vec::new(),
            current_frame: None
        }
    }
    pub fn queue_position(&mut self, position: PolarCoordinate) {
        let prev_position = match &self.checkpoints.last() {
            None => &PolarCoordinate {
                theta: 0.0,
                rho: 0.0,
            },
            checkpoint_opt => &checkpoint_opt.unwrap().position,
        };

        if prev_position.equals(&position) {
            return;
        }
        let next_checkpoint_index  = match self.checkpoints.len() {
            0 => 0,
            _ => self.current_checkpoint_index + 1
        };

        self.checkpoints.push(Checkpoint {
            index: next_checkpoint_index,
            position: position.copy(),
            vector: prev_position.vector_to(&position),
            total_distance: prev_position.distance(&position),
        });
    }

    pub fn next_frame(&mut self) -> MotionFrame {
        let current_checkpoint = &self.checkpoints[self.current_checkpoint_index];
        let motion_frame = match &self.current_frame {
            None => MotionFrame {
                position: PolarCoordinate { theta: 0.0, rho: 0.0 },
                speed: 0.0,
                total_distance: 0.0,
            },

            prev_frame_opt => {
                let prev_frame = prev_frame_opt.as_ref().unwrap();
                let scaled_vector = &current_checkpoint.vector.multiply(STEP_DISTANCE);
                let next_position = PolarCoordinate {
                    theta: prev_frame.position.theta + scaled_vector.theta,
                    rho: prev_frame.position.rho + scaled_vector.rho,
                };

                MotionFrame {
                    position: next_position.copy(),
                    speed: (prev_frame.speed + MAX_ACCELERATION).min(MAX_SPEED),
                    total_distance: prev_frame.total_distance
                        + prev_frame.position.distance(&next_position),
                }
            }
        };

        self.current_frame = Some(motion_frame.copy());
        // TODO: we need to figure out when to decelerate
        if motion_frame.total_distance >= current_checkpoint.total_distance {
           self.current_checkpoint_index += 1;
        }
        motion_frame
    }
    fn next_direction_change(&self) -> usize {
        let start_checkpoint: &Checkpoint = &self.checkpoints[self.current_checkpoint_index];
        let start_direction = start_checkpoint.vector.direction();
        for i in self.current_checkpoint_index..self.checkpoints.len() {
            let current_checkpoint = &self.checkpoints[i];
            if !start_direction.equals(&current_checkpoint.vector.direction()) {
                return i;
            }
        }

        self.checkpoints.len() - 1
    }
}

