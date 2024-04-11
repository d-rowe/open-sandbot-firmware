#include <Stepper.h>
// number of steps on your motor
#define STEPS 200

const char TR_DELIMITER = ' ';
// commands
const String SPEED_COMMAND = "SPEED:";
const String PAUSE_COMMAND = "PAUSE";
const String RESUME_COMMAND = "RESUME";
const String HOME_COMMAND = "HOME";
const String MOVE_COMMAND = "MOVE:";
// statuses
const String IDLE_STATUS = "STATUS:IDLE";
const String MOVING_STATUS = "STATUS:MOVING";
// errors
const String ERR_UNKNOWN_COMMAND = "ERR_UNKNOWN_COMMAND";
// primary motor driver pins
const int AIN2 = 18;
const int AIN1 = 19;
const int BIN1 = 20;
const int BIN2 = 21;
// secondary motor driver pins
const int AIN4 = 10;
const int AIN3 = 11;
const int BIN3 = 12;
const int BIN4 = 13;
// hal effect sensor pins for homing
const int HAL1 = 14;
const int HAL2 = 15;

int primary_steps = 0;
int secondary_steps = 0;
int primary_steps_target = 0;
int secondary_steps_target = 0;
bool is_homing = false;

Stepper primary_stepper(STEPS, AIN2, AIN1, BIN1, BIN2);
Stepper secondary_stepper(STEPS, AIN4, AIN3, BIN3, BIN4);
bool movement_complete = false;
bool paused = false;

void setup() {
  Serial.begin(115200);
  Serial.setTimeout(2);
  pinMode(HAL1, INPUT);
  pinMode(HAL2, INPUT);
  primary_stepper.setSpeed(100);
  secondary_stepper.setSpeed(100);
  // home on power on
  is_homing = true;
}

void loop() {
  if (Serial.available() > 0) {
    String command = Serial.readString();
    parseCommand(command);
  }

  if (!paused) {
    progressMovement();
  }
}

void parseCommand(String command) {
  if (command.startsWith(MOVE_COMMAND)) {
    move(command);
    return;
  }

  if (command.startsWith(SPEED_COMMAND)) {
    setSpeed(command);
    return;
  }

  if (command.startsWith(PAUSE_COMMAND)) {
    paused = true;
    return;
  }

  if (command.startsWith(RESUME_COMMAND)) {
    paused = false;
    if (movement_complete) {
      sendMessage(IDLE_STATUS);
    }
    return;
  }

  sendMessage(ERR_UNKNOWN_COMMAND);
} 

void setSpeed(String command) {
  int speed = command.substring(SPEED_COMMAND.length()).toInt();
  primary_stepper.setSpeed(speed);
  secondary_stepper.setSpeed(speed);
}

void move(String command) {
  String thetaRho = command.substring(MOVE_COMMAND.length());
  int delimiterIndex = thetaRho.indexOf(TR_DELIMITER);
  double theta = thetaRho.substring(0, delimiterIndex).toFloat();
  double rho = thetaRho.substring(delimiterIndex + 1).toFloat();

  moveToThetaRho(theta, rho);
}

void sendMessage(String message) {
  Serial.println(message);
}
