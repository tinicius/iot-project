import dotenv from 'dotenv';
import config from 'config';
import { MQTTMessaging } from './infra/impl/mqtt_messaging';
import { RandomGenerator } from './infra/impl/random_generator';
import { TemperatureService } from './services/impl/temperature_service';
import { HumidityService } from './services/impl/humidity_service';
import { StatusService } from './services/impl/status';
import { logger } from './utils/logger';

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

    const batteryVoltageGenerator = new RandomGenerator(0, 5);
    const signalGenerator = new RandomGenerator(0, 100);
    const statusService = new StatusService(
        batteryVoltageGenerator,
        signalGenerator,
        messaging
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
    if (
        !config.has('INTERVAL.TEMPERATURE') ||
        !config.has('INTERVAL.HUMIDITY') ||
        !config.has('INTERVAL.STATUS')
    ) {
        logger.error('Invalid interval!');

        return {
            humidityTimeInterval: 0,
            statusTimeInterval: 0,
            temperatureTimeInterval: 0,
        };
    }

    return {
        temperatureTimeInterval: Number(config.get('INTERVAL.TEMPERATURE')),
        humidityTimeInterval: Number(config.get('INTERVAL.HUMIDITY')),
        statusTimeInterval: Number(config.get('INTERVAL.STATUS')),
    };
}
