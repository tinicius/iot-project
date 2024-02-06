import { HealthyData } from '../../entities/healthy_data';
import { IGenerator } from '../../interfaces/infra/generator';
import { IMessaging } from '../../interfaces/infra/messaging';
import { IHealthy } from '../../interfaces/services/healthy';
import os from 'os';

export class HealthyService implements IHealthy {
    batteryGenerator: IGenerator;
    messaging: IMessaging;

    constructor(batteryGenerator: IGenerator, messaging: IMessaging) {
        this.batteryGenerator = batteryGenerator;
        this.messaging = messaging;
    }

    send(): void {
        const batteryVoltage = this.batteryGenerator.generate();

        const data: HealthyData = {
            batteryVoltage,
            services: ['TEMP', 'HUMIDITY'],
            time: Date.now(),
            device: os.userInfo().username,
        };

        const topic = `IoTProject/healthy/${data.device}`;

        this.messaging.publishHealthy(topic, data);
    }
}
