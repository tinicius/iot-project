import { MQTTMessaging } from "./impl/infra/mqtt_messaging";
import { RandomGenerator } from "./impl/infra/random_generator";
import { HumidityService } from "./impl/services/humidity";
import { TemperatureService } from "./impl/services/temperature";
import dotenv from "dotenv";

(() => {

    dotenv.config({
        path: `.env`,
    });
   
    const messaging = new MQTTMessaging();

    const temperatureGenerator = new RandomGenerator(10, 100);
    const temperatureService = new TemperatureService(temperatureGenerator, messaging);

    const humidityGenerator = new RandomGenerator(0, 1);
    const humidityService = new HumidityService(humidityGenerator, messaging);

    const { temperatureTimeInterval, humidityTimeInterval } = getTimeInterval();
  
    setInterval(() => temperatureService.send(), temperatureTimeInterval);
    setInterval(() => humidityService.send(), humidityTimeInterval);

})();

function getTimeInterval(): {
    temperatureTimeInterval: number;
    humidityTimeInterval: number;
  } {
    const temperatureInterval = process.env.TEMPERATURE_INTERVAL;
    const humidityInterval = process.env.HUMIDITY_INTERVAL;
  
    const temperatureTimeInterval = temperatureInterval
      ? Number(temperatureInterval)
      : 0;
  
    const humidityTimeInterval = humidityInterval ? Number(humidityInterval) : 0;
  
    return {
      temperatureTimeInterval,
      humidityTimeInterval,
    };
  }