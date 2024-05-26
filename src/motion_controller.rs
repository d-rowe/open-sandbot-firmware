use crate::coordinate::PolarCoordinate;

struct Checkpoint {
    index: i16,
    position: PolarCoordinate,
    vector: PolarCoordinate,
    total_distance: f64,
}

impl Checkpoint {
    fn has_direction_changed(&self, other: &Checkpoint) -> bool {
        !self.vector.equals(&other.vector)
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
    current_checkpoint_index: i16,
    checkpoints: Vec<Checkpoint>,
    last_frame: Option<MotionFrame>,
}

impl MotionController {
    pub fn new() -> Self {
        let mut checkpoints: Vec<Checkpoint> = Vec::new();
        checkpoints.push(Checkpoint {
            index: 0,
            position: PolarCoordinate { theta: 0.0, rho: 0.0 },
            vector: PolarCoordinate { theta: 0.0, rho: 0.0},
            total_distance: 0.0,
        });
        MotionController {
            current_checkpoint_index: 0,
            checkpoints,
            last_frame: None
        }
    }
    pub fn queue_position(&mut self, position: PolarCoordinate) {
        let prev_position = &self.checkpoints.last().unwrap().position;
        if prev_position.equals(&position) {
            return;
        }

        self.checkpoints.push(Checkpoint {
            index: self.current_checkpoint_index + 1,
            position: position.copy(),
            vector: prev_position.vector_to(&position),
            total_distance: prev_position.distance(&position),
        });
    }

    pub fn next_frame(&mut self) -> MotionFrame {
        let first_checkpoint = self.checkpoints.first().unwrap();

        let motion_frame = match &self.last_frame {
            None => MotionFrame {
                position: first_checkpoint.position.copy(),
                speed: 0.0,
                total_distance: first_checkpoint.total_distance,
            },
            last_frame => MotionFrame {
                position: PolarCoordinate { theta: 0.0, rho: 0.0 },
                speed:  last_frame.as_ref().unwrap().speed + 0.1,
                total_distance:  last_frame.as_ref().unwrap().total_distance + 0.1,
            }
        };

        self.last_frame = Some(motion_frame.copy());
        motion_frame
    }
}

