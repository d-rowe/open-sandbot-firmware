const int MAIN_PULLEY_TEETH = 90;
const int MOTOR_PULLEY_TEETH = 14;
const int DEGREES_PER_STEP = 1.8;
const int STEPS_PER_DEG = MAIN_PULLEY_TEETH / MOTOR_PULLEY_TEETH / DEGREES_PER_STEP / 2;

double partial_step = 0;

void moveToThetaRho(double theta, double rho) {
  const double theta_degrees = degrees(theta);
  const double secondary_degrees = 180.0 - degrees(acos((0.5 - pow(rho, 2.0)) * 2.0));
  const double primary_offset = secondary_degrees / 2;
  const double primary_degrees = theta_degrees - primary_offset;

  setTargetPosition(primary_degrees, secondary_degrees);
}

void progressMovement() {
  if (is_homing) {
    progressHome();
    return;
  }

  int primary_step_delta = primary_steps_target - primary_steps;
  int primary_step_delta_abs = abs(primary_step_delta);
  int secondary_step_delta = secondary_steps_target - secondary_steps;
  int secondary_step_delta_abs = abs(secondary_step_delta);

  if (primary_step_delta == 0 && secondary_step_delta == 0) {
    partial_step = 0;
    if (!movement_complete) {
      movement_complete = true;
      sendMessage(IDLE_STATUS);
    }
    return;
  }

  if (movement_complete) {
    sendMessage(MOVING_STATUS);
    movement_complete = false;
  }

  bool is_primary_faster = primary_step_delta_abs > secondary_step_delta_abs;
  if (is_primary_faster) {
    int primary_direction = primary_step_delta / primary_step_delta_abs;
    int secondary_direction = secondary_step_delta / secondary_step_delta_abs;

    primaryStep(primary_direction);
    
    double speed_ratio = (double) secondary_step_delta_abs / (double) primary_step_delta_abs;
    partial_step += speed_ratio;
  
    while (partial_step >= 0) {
      secondaryStep(secondary_direction);
      partial_step -= 1;
    }
  } else {
    int primary_direction = primary_step_delta / primary_step_delta_abs;
    int secondary_direction = secondary_step_delta / secondary_step_delta_abs;
  
    secondaryStep(secondary_direction);
    
    double speed_ratio = (double) primary_step_delta_abs / (double) secondary_step_delta_abs;
    partial_step += speed_ratio;
  
    while (partial_step >= 0) {
      primaryStep(primary_direction);
      partial_step -= 1;
    }
  }
}

void progressHome() {
  const bool is_primary_home = isHallSensorActivated(HAL1);
  const bool is_secondary_home = isHallSensorActivated(HAL2);
  if (!is_primary_home) {
    primary_stepper.step(1);
    return;
  }

  if (!is_secondary_home) {
    secondary_stepper.step(1);
    return;
  }

  primary_steps = 0;
  secondary_steps = 0;
  is_homing = false;
  setSpeed(STEPPER_SPEED_DEFAULT);
  sendMessage(IDLE_STATUS);
}

void primaryStep(int steps) {
  primary_stepper.step(steps);
  primary_steps += steps;
}

void secondaryStep(int steps) {
  secondary_stepper.step(steps);
  secondary_steps += steps;
}

void setTargetPosition(double primary_deg, double secondary_deg) {
  primary_steps_target = round(primary_deg * STEPS_PER_DEG);
  secondary_steps_target = round(secondary_deg * STEPS_PER_DEG) + primary_steps_target;
}

bool isHallSensorActivated(int pin) {
  return digitalRead(pin) == 0;
}
