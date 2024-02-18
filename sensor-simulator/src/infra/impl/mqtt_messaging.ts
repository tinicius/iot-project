import mqtt, { MqttClient } from 'mqtt';
import { Service } from '../../entities/service';
import { Status } from '../../entities/status';
import { IMessaging } from '../messaging';
import { logger } from '../../utils/logger';

interface MQTTConfigs {
    user: string;
    password: string;
    protocol: string;
    host: string;
    port: number;
    clientId: string;
}

export class MQTTMessaging implements IMessaging {
    private client: MqttClient;

    constructor() {
        this.connect();
    }

    publishService(data: Service): void {
        const topic = `IoTProject/services/${data.device}/${data.type}`;

        this.client.publish(
            topic,
            JSON.stringify({
                value: data.value,
                time: data.time,
            })
        );
    }

    publishStatus(data: Status): void {
        const topic = `IoTProject/status/${data.device}`;

        this.client.publish(
            topic,
            JSON.stringify({
                time: data.time,
                batteryVoltage: data.batteryVoltage,
                signal: data.signal,
            })
        );
    }

    private connect(): void {
        const { user, password, protocol, host, port, clientId } = this.envs();

        try {
            this.client = mqtt.connect(`${protocol}://${host}`, {
                port,
                clientId,
                username: user,
                password,
            });
        } catch (error) {
            logger.error('Error on connect in MQTT!');
            return;
        }

        logger.info(`Connected in: ${protocol}://${host}`);
    }

    private envs(): MQTTConfigs {
        const user = process.env.MQTT_USER;
        const password = process.env.MQTT_PASSWORD;
        const protocol = process.env.MQTT_PROTOCOL;
        const host = process.env.MQTT_HOST;
        const port = process.env.MQTT_PORT;
        const clientId = process.env.MQTT_CLIENT_ID;

        if (!user || !password || !host || !protocol || !port || !clientId) {
            throw new Error('Invalid mqtt credentials!');
        }

        if (isNaN(Number(port))) {
            throw new Error('Invalid mqtt port!');
        }

        return { user, password, protocol, host, port: Number(port), clientId };
    }
}
