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

    pub fn vector_to(&self, other: &PolarCoordinate) -> PolarCoordinate {
        let theta_delta = other.theta - self.theta;
        let rho_delta = other.rho - self.rho;
        let normalization_factor = theta_delta.abs().max(rho_delta.abs());

        PolarCoordinate {
            theta: theta_delta / normalization_factor,
            rho: rho_delta / normalization_factor,
        }
    }

    pub fn scale(&self, factor: f64) -> PolarCoordinate {
        PolarCoordinate {
            theta: self.theta * factor,
            rho: self.rho * factor,
        }
    }

    pub fn direction(&self) -> PolarCoordinate {
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

