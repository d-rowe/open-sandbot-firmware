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
        let mut theta_direction: f64;
        if (theta_delta == 0.0) {
            theta_direction = 0.0;
        } else {
            theta_direction = theta_delta / theta_delta.abs();
        }
        let mut rho_direction: f64;
        if (rho_delta == 0.0) {
            rho_direction = 0.0;
        } else {
            rho_direction = rho_delta / rho_delta.abs();
        }

        PolarCoordinate {
            theta: theta_direction,
            rho: rho_direction,
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

