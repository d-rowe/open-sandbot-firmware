char TR_DELIMITER = ' ';
String MOVE_COMMAND = "MOVE:";
String MOVE_DONE = "MOVE_DONE";

void setup() {
  Serial.begin(9600);
}

void loop() {
  if (Serial.available() == 0) {
    return;
  }

  String command = Serial.readString();
  parseCommand(command);
}

void parseCommand(String command) {
  if (command.startsWith(MOVE_COMMAND)) {
    move(command);
    sendMessage(MOVE_DONE);
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
