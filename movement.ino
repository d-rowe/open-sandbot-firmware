const int MAIN_PULLEY_TEETH = 60;
const int MOTOR_PULLEY_TEETH = 14;
const int DEGREES_PER_STEP = 1.8;
const int STEPS_PER_DEG = MAIN_PULLEY_TEETH / MOTOR_PULLEY_TEETH / DEGREES_PER_STEP / 2;

int primary_steps = 0;
int secondary_steps = 0;
int primary_steps_target = 0;
int secondary_steps_target = 0;

void moveToThetaRho(float theta, float rho) {
  const float theta_degrees = degrees(theta);
  const float primary_degrees = 180 - degrees(acos((0.5 - pow(rho, 2)) * 2));
  const float secondary_offset = primary_degrees / 2;
  const float secondary_degrees = theta_degrees - secondary_offset;
  setTargetPosition(primary_degrees, secondary_degrees);
}

void progressMovement() {
  if (primary_steps < primary_steps_target) {
    primary_steps++;
    Serial.println(primary_steps);
  }
}

void setTargetPosition(float primary_deg, float secondary_deg) {
  primary_steps_target = round(primary_deg * STEPS_PER_DEG);
  secondary_steps_target = round(secondary_deg * STEPS_PER_DEG) + primary_steps_target;
  Serial.println(primary_steps_target);
  Serial.println(secondary_steps_target);
}
