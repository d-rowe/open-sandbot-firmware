pub struct PolarCoordinate {
    pub theta: f64,
    pub rho: f64,
}

impl PolarCoordinate {
    pub fn distance(&self, other: &PolarCoordinate) -> f64 {
        let avg_rho = (other.rho + self.rho) / 2.0;
        let theta_distance = (other.theta - self.theta).abs() * avg_rho;
        let rho_distance = (other.rho - self.rho).abs();
        theta_distance + rho_distance
    }

    pub fn scale(&self, factor: f64) -> PolarCoordinate {
        PolarCoordinate {
            theta: self.theta * factor,
            rho: self.rho * factor,
        }
    }

    pub fn add(&self, other: &PolarCoordinate) -> Self {
        PolarCoordinate {
            theta: self.theta + other.theta,
            rho: self.rho + other.rho,
        }
    }

    pub fn subtract(&self, other: &PolarCoordinate) -> Self {
        PolarCoordinate {
            theta: self.theta - other.theta,
            rho: self.rho - other.rho,
        }
    }

    pub fn direction(&self) -> Self {
        PolarCoordinate {
            theta: direction(self.theta),
            rho: direction(self.rho),
        }
    }

    pub fn copy(&self) -> Self {
       PolarCoordinate {
           theta: self.theta,
           rho: self.rho,
       }
    }

    pub fn equals(&self, other: &PolarCoordinate) -> bool {
        self.theta == other.theta && self.rho == other.rho
    }
}

fn direction(value: f64) -> f64 {
    match value {
        0.0 => 0.0,
        val => val / val.abs(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polar_coordinate_equals() {
        let a = PolarCoordinate { theta: 0.2, rho: 0.3 };
        let b = PolarCoordinate { theta: 0.2, rho: 0.3 };
        let c = PolarCoordinate { theta: 0.2, rho: 0.4 };
        let d = PolarCoordinate { theta: 0.1, rho: 0.3 };
        assert_eq!(a.equals(&b), true);
        assert_eq!(a.equals(&c), false);
        assert_eq!(a.equals(&d), false);
    }
}

