use crate::coordinate::PolarCoordinate;


#[derive(Clone)]
pub struct MotionFrame {
    pub position: PolarCoordinate,
    pub speed: f64,
    pub relative_distance: f64,
    pub absolute_distance: f64,
}