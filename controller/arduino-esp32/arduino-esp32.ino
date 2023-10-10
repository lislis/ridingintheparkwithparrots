#include <Adafruit_MPU6050.h>
#include <Adafruit_Sensor.h>
#include <Wire.h>

Adafruit_MPU6050 mpu;

sensors_event_t a, g, temp;

void setup(void) {
  Serial.begin(115200);

  if (!mpu.begin()) {
    Serial.println("Failed to find MPU6050 chip");
    while (1) {
      delay(10);
    }
  }
  Serial.println("MPU6050 Found!");
  mpu.setGyroRange(MPU6050_RANGE_250_DEG);

  Serial.println("");
  delay(100);
}

void loop() {
  mpu.getEvent(&a, &g, &temp);

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
  //Serial.print("GyroY:");
  Serial.println(g.gyro.y);
  // Serial.print(",");
  // Serial.print("GyroZ:");
  // Serial.print(g.gyro.z);
  // Serial.println("");

  delay(100);
}
