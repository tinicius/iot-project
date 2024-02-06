import { MQTTMessaging } from './impl/infra/mqtt_messaging';
import { RandomGenerator } from './impl/infra/random_generator';
import { HealthyService } from './impl/services/healthy';
import { HumidityService } from './impl/services/humidity';
import { TemperatureService } from './impl/services/temperature';
import dotenv from 'dotenv';

(() => {
    dotenv.config({
        path: `.env`,
    });

    const messaging = new MQTTMessaging();

    const temperatureGenerator = new RandomGenerator(10, 100);
    const temperatureService = new TemperatureService(
        temperatureGenerator,
        messaging
    );

    const humidityGenerator = new RandomGenerator(0, 1);
    const humidityService = new HumidityService(humidityGenerator, messaging);

    const healthyGenerator = new RandomGenerator(0, 5);
    const healthyService = new HealthyService(healthyGenerator, messaging);

    const {
        temperatureTimeInterval,
        humidityTimeInterval,
        healthyTimeInterval,
    } = getTimeInterval();

    setInterval(() => temperatureService.send(), temperatureTimeInterval);
    setInterval(() => humidityService.send(), humidityTimeInterval);
    setInterval(() => healthyService.send(), healthyTimeInterval);
})();

function getTimeInterval(): {
    temperatureTimeInterval: number;
    humidityTimeInterval: number;
    healthyTimeInterval: number;
} {
    const temperatureInterval = process.env.TEMPERATURE_INTERVAL;
    const humidityInterval = process.env.HUMIDITY_INTERVAL;
    const healthyInterval = process.env.HEALTHY_INTERVAL;

    const temperatureTimeInterval = temperatureInterval
        ? Number(temperatureInterval)
        : 0;

    const humidityTimeInterval = humidityInterval
        ? Number(humidityInterval)
        : 0;

    const healthyTimeInterval = healthyInterval ? Number(healthyInterval) : 0;

    return {
        temperatureTimeInterval,
        humidityTimeInterval,
        healthyTimeInterval,
    };
}
