const char TR_DELIMITER = ' ';
const String MOVE_COMMAND = "MOVE:";
const String MOVE_DONE = "MOVE_DONE";
const String ERROR_MOVE_IN_PROGRESS = "ERROR_MOVE_IN_PROGRESS";
bool is_move_in_progress = false;

void setup() {
  Serial.begin(9600);
}

void loop() {
  if (Serial.available() > 0) {
    String command = Serial.readString();
    parseCommand(command);
  }

  progressMovement();
}

void parseCommand(String command) {
  if (command.startsWith(MOVE_COMMAND)) {
    if (is_move_in_progress) {
      sendMessage(ERROR_MOVE_IN_PROGRESS);
      return;
    }

    move(command);
  }
}

void move(String command) {
  String thetaRho = command.substring(MOVE_COMMAND.length());
  int delimiterIndex = thetaRho.indexOf(TR_DELIMITER);
  float theta = thetaRho.substring(0, delimiterIndex).toFloat();
  float rho = thetaRho.substring(delimiterIndex + 1).toFloat();

  moveToThetaRho(theta, rho);
}

void sendMessage(String message) {
  Serial.println(message);
}
