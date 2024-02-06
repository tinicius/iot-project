import mqtt, { MqttClient } from "mqtt";
import { AlgorithmData } from "../../entities/algorithm_data";
import { IMessaging } from "../../interfaces/infra/messaging";
import { logger } from "../../utils/logger";

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

    publish(data: AlgorithmData): void {
        
        const topic = `IoTProject/${data.device}/${data.type}`;

        this.client.publish(topic, JSON.stringify({
            value: data.value,
            time: data.time
        }));

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
          logger.error((error as Error).message);
          throw error;
        }
    
        logger.info(`Connecting in: ${protocol}://${host}`);
      }
    
      private envs(): MQTTConfigs {
        const user = process.env.MQTT_USER;
        const password = process.env.MQTT_PASSWORD;
        const protocol = process.env.MQTT_PROTOCOL;
        const host = process.env.MQTT_HOST;
        const port = process.env.MQTT_PORT;
        const clientId = process.env.MQTT_CLIENT_ID;
    
        if (!user || !password || !host || !protocol || !port || !clientId) {
          throw new Error("Invalid mqtt credentials!");
        }
    
        if (isNaN(Number(port))) {
          throw new Error("Invalid mqtt port!");
        }
    
        return { user, password, protocol, host, port: Number(port), clientId };
      }
    
}