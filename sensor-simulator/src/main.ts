import dotenv from 'dotenv';
import { MQTTMessaging } from './infra/impl/mqtt_messaging';
import { RandomGenerator } from './infra/impl/random_generator';
import { TemperatureService } from './services/impl/temperature_service';
import { HumidityService } from './services/impl/humidity_service';
import { StatusService } from './services/impl/status';

(() => {
    dotenv.config({
        path: `.env`,
    });

    const device = `device ${Math.floor(Math.random() * 100)}`;

    const messaging = new MQTTMessaging();

    const temperatureGenerator = new RandomGenerator(10, 100);
    const temperatureService = new TemperatureService(
        temperatureGenerator,
        messaging,
        device
    );

    const humidityGenerator = new RandomGenerator(0, 1);
    const humidityService = new HumidityService(
        humidityGenerator,
        messaging,
        device
    );

    const batteryVoltageGenerator = new RandomGenerator(0, 5);
    const signalGenerator = new RandomGenerator(0, 100);
    const statusService = new StatusService(
        batteryVoltageGenerator,
        signalGenerator,
        messaging,
        device
    );

    const {
        temperatureTimeInterval,
        humidityTimeInterval,
        statusTimeInterval,
    } = getTimeInterval();

    setInterval(() => temperatureService.send(), temperatureTimeInterval);
    setInterval(() => humidityService.send(), humidityTimeInterval);
    setInterval(() => statusService.send(), statusTimeInterval);
})();

function getTimeInterval(): {
    temperatureTimeInterval: number;
    humidityTimeInterval: number;
    statusTimeInterval: number;
} {
    return {
        temperatureTimeInterval: 10000,
        humidityTimeInterval: 12000,
        statusTimeInterval: 8000,
    };
}
