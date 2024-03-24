int MAIN_PULLEY_TEETH = 60;
int MOTOR_PULLEY_TEETH = 14;
int DEGREES_PER_STEP = 1.8;

int primary_steps = 0;
int secondary_steps = 0;

void moveToThetaRho(float theta, float rho) {
  float deg = degrees(theta);
  float secondary_deg = 180 - degrees(acos((pow(0.5, 2) * 2) - pow(rho, 2) / 0.5));
  float primary_offset_deg = secondary_deg / 2;
  float primary_deg = deg - primary_offset_deg;
  moveToAngles(primary_deg, secondary_deg);
}

void moveToAngles(float primary_deg, float secondary_deg) {
  int primary_steps_target = round(primary_deg * DEGREES_PER_STEP);
  int secondary_steps_target = round(secondary_deg * DEGREES_PER_STEP);

  int primary_steps_required = primary_steps_target - primary_steps;
  int secondary_steps_required = secondary_steps_target - secondary_steps;

  Serial.println(primary_steps_required);

  for (int step = 0; step < 10; step++) {
    primary_steps++;
    Serial.print("step ");
    Serial.println(step);
  }
}
