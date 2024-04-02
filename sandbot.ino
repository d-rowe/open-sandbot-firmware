#include <Stepper.h>
// number of steps on your motor
#define STEPS 200

const char TR_DELIMITER = ' ';
const String SPEED_COMMAND = "SPEED:";
const String SPEED_ACK = "SPEED_ACK";
const String PAUSE_COMMAND = "PAUSE";
const String PAUSE_ACK = "PAUSE_ACK";
const String RESUME_COMMAND = "RESUME";
const String RESUME_ACK = "RESUME_ACK";
const String MOVE_COMMAND = "MOVE:";
const String MOVE_ACK = "MOVE_ACK";
const String MOVE_DONE = "MOVE_DONE";
const String ERR_UNKNOWN_COMMAND = "ERR_UNKNOWN_COMMAND";
// primary motor driver pins
const int AIN2 = 18;
const int AIN1 = 19;
const int BIN1 = 20;
const int BIN2 = 21;
const int AIN4 = 10;
const int AIN3 = 11;
const int BIN3 = 12;
const int BIN4 = 13;

Stepper primary_stepper(STEPS, AIN2, AIN1, BIN1, BIN2);
Stepper secondary_stepper(STEPS, AIN4, AIN3, BIN3, BIN4);
bool movement_complete = false;
bool paused = false;

void setup() {
  Serial.begin(115200);
  Serial.setTimeout(2);
  primary_stepper.setSpeed(50);
  secondary_stepper.setSpeed(50);
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
    sendMessage(MOVE_ACK);
    move(command);
    return;
  }

  if (command.startsWith(SPEED_COMMAND)) {
    setSpeed(command);
    sendMessage(SPEED_ACK);
    return;
  }

  if (command.startsWith(PAUSE_COMMAND)) {
    paused = true;
    sendMessage(PAUSE_ACK);
    return;
  }

  if (command.startsWith(RESUME_COMMAND)) {
    paused = false;
    sendMessage(RESUME_ACK);
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
