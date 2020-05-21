#include "bqLiquidCrystal.h"
#include "protocol.h"

/*
  Serial Call and Response
  Language: Wiring/Arduino

  This program sends an ASCII A (byte of value 65) on startup and repeats that
  until it gets some data in. Then it waits for a byte in the serial port, and
  sends three sensor values whenever it gets a byte in.

  The circuit:
  - potentiometers attached to analog inputs 0 and 1
  - pushbutton attached to digital I/O 2

  created 26 Sep 2005
  by Tom Igoe
  modified 24 Apr 2012
  by Tom Igoe and Scott Fitzgerald
  Thanks to Greg Shakar and Scott Fitzgerald for the improvements

  This example code is in the public domain.

  http://www.arduino.cc/en/Tutorial/SerialCallResponse
*/

int firstSensor = 0;  // first analog sensor
int secondSensor = 0; // second analog sensor
int thirdSensor = 0;  // digital sensor
int inByte = 0;       // incoming serial byte

LiquidCrystal lcd(0); // we create an LCD object

void lcdWriteCommand(LcdwriteVal v)
{
  lcd.setCursor(0, 0);
  LCDWrite(v.l1);
  lcd.setCursor(0, 1);
  LCDWrite(v.l2);
}

void ledOn(Led_1Val v)
{
  digitalWrite(13, v.state);
}

void handshake(HandshakeVal v)
{
  RecievedVal r;
  Serial.print(repr_RecievedVal(r));
}

void recieved(RecievedVal v) {}

void lcd_clear(LcdclearVal v) {
  lcd.clear();
  lcd.home();
  delay(100);
}

Parser parser(lcdWriteCommand, lcd_clear, ledOn, handshake, recieved);

void setup()
{
  // start serial port at 9600 bps:
  Serial.begin(9600);
  while (!Serial)
  {
    ; // wait for serial port to connect. Needed for native USB port only
  }

  lcd.begin(16, 2);
  byte personaje[8] = {
      B01110,
      B01010,
      B01110,
      B00100,
      B11111,
      B00100,
      B01010,
      B10001
  };
  lcd.createChar(0, personaje);
  lcd.clear();
  delay(500);
  lcd.write(0);

  pinMode(13, OUTPUT); // digital sensor is on digital pin 2
  pinMode(12, INPUT);
  digitalWrite(13, HIGH);
  delay(500);
  digitalWrite(13, LOW);
  HandshakeVal r;
  LCDWrite("Connecting");
  lcd.home();
  
  Serial.print(repr_HandshakeVal(r));


  while (Serial.available() <= 0)
  {
  }
}

int last = 1;
void loop()
{
  // if we get a valid byte, read analog ins:
  if (Serial.available() > 0)
  {
    // get incoming byte:
    inByte = Serial.read();
    parser.parse(inByte);
    // if (char(inByte) == 'A') {
    //   digitalWrite(13, HIGH);
    //   lcd.home();
    //   LCDWrite("HIGH");
    // }
    // if (char(inByte) == 'B') {
    //   digitalWrite(13, LOW);
    //   lcd.home();
    //   LCDWrite("LOW ");
    // }
  }
}

void LCDWrite(String s)
{
  for (int i = 0; i < s.length(); i++)
  {
    lcd.write(s.charAt(i));
    delay(10);
  }
}
