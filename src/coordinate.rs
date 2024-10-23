use libm::fabs;

#[derive(Clone)]
pub struct PolarCoordinate {
    pub theta: f64,
    pub rho: f64,
}

impl PolarCoordinate {
    pub fn distance(&self, other: &PolarCoordinate) -> f64 {
        let avg_rho = (other.rho + self.rho) / 2.0;
        let theta_distance = fabs(other.theta - self.theta) * avg_rho;
        let rho_distance = fabs(other.rho - self.rho);
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

    pub fn equals(&self, other: &PolarCoordinate) -> bool {
        self.theta == other.theta && self.rho == other.rho
    }
}

fn direction(value: f64) -> f64 {
    match value {
        0.0 => 0.0,
        val => val / fabs(val),
    }
}
