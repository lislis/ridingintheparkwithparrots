#include <WebServer.h>
#include <WiFi.h>
#include <WiFiClient.h>

#include <Adafruit_MPU6050.h>
#include <Adafruit_Sensor.h>
#include <Wire.h>

Adafruit_MPU6050 mpu;

sensors_event_t a, g, temp;
bool left, right;


WebServer server ( 80 );

const char* ssid     = "...";
const char* password = "...";

void handleRoot() { 
  //server.send( 200, "application/json", getPayload() ); 
  server.send( 200, "text/plain", getText() ); 
}

String getText() {
  String text = "";
  if (left && !right) {
    text += "left";
  } else if (right && !left) {
    text += "right";
  } else {
    text += "none";
  }
  return text;
}

String getPayload() {
  String payload = "{\"x:\": ";
  payload += g.gyro.x;
  payload += ", \"y\": ";
  payload += g.gyro.y;
  payload += ", \"z\": ";
  payload += g.gyro.z;
  payload += "}";
  return payload;
}

void setup(void) {
  Serial.begin(115200);
  WiFi.mode(WIFI_STA);
  WiFi.begin(ssid, password);
  Serial.println("");
  
  while (WiFi.status() != WL_CONNECTED) {
        delay(500);
        Serial.print(".");
    }
  Serial.println("");
  Serial.print("Connected to");
  Serial.println(ssid);
  Serial.println("IP address: ");
  Serial.println(WiFi.localIP());

  server.on ( "/", handleRoot );
  server.begin();
  Serial.println ( "HTTP server started" );

  if (!mpu.begin()) {
    Serial.println("Failed to find MPU6050 chip");
    while (1) {
      delay(10);
    }
  }
  Serial.println("MPU6050 Found!");
  mpu.setGyroRange(MPU6050_RANGE_250_DEG);
  mpu.setHighPassFilter(MPU6050_HIGHPASS_0_63_HZ);
  mpu.setMotionDetectionThreshold(1);
  mpu.setMotionDetectionDuration(20);
  mpu.setInterruptPinLatch(true);	// Keep it latched.  Will turn off when reinitialized.
  mpu.setInterruptPinPolarity(true);
  mpu.setMotionInterrupt(true);


  Serial.println("");
  delay(100);
}

void loop() {
  server.handleClient();

  if(mpu.getMotionInterruptStatus()) {
    /* Get new sensor events with the readings */
    sensors_event_t a, g, temp;
    mpu.getEvent(&a, &g, &temp);

    // better 
    // but
    // 


    if (g.gyro.y > 0.5) {
      left = true;
      right = false;
    } else if (g.gyro.y < -0.5) {
      right = true;
      left = false;
    } else {
      right = false;
      left = false;
    }

    /* Print out the values */
    // Serial.print("AccelX:");
    // Serial.print(a.acceleration.x);
    // Serial.print(",");
    // Serial.print("AccelY:");
    // Serial.print(a.acceleration.y);
    // Serial.print(",");
    // Serial.print("AccelZ:");
    // Serial.print(a.acceleration.z);
    // Serial.print(", ");
    // Serial.print("GyroX:");
    // Serial.print(g.gyro.x);
    // Serial.print(",");
    Serial.print("GyroY:");
    Serial.print(g.gyro.y);
    // Serial.print(",");
    // Serial.print("GyroZ:");
    // Serial.print(g.gyro.z);
    // Serial.println("");
  } else {
    //Serial.print("none");
  }
  // Serial.print(g.gyro.x);
  // Serial.print(",");
  // Serial.print(g.gyro.y);
  // Serial.print(",");
  // Serial.print(g.gyro.z);
  delay(100);
}




