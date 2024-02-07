import { HealthyData } from '../../entities/healthy_data';
import { IGenerator } from '../../interfaces/infra/generator';
import { IMessaging } from '../../interfaces/infra/messaging';
import { IHealthy } from '../../interfaces/services/healthy';

export class HealthyService implements IHealthy {
    batteryGenerator: IGenerator;
    messaging: IMessaging;

    constructor(batteryGenerator: IGenerator, messaging: IMessaging) {
        this.batteryGenerator = batteryGenerator;
        this.messaging = messaging;
    }

    send(): void {
        const batteryVoltage = this.batteryGenerator.generate();
        const device = this.getDevice();

        const data: HealthyData = {
            batteryVoltage,
            services: ['TEMP', 'HUMIDITY'],
            time: Date.now(),
            device,
        };

        const topic = `IoTProject/healthy/${data.device}`;

        this.messaging.publishHealthy(topic, data);
    }

    private getDevice(): string {
        const deviceName = process.env.DEVICE;

        if (!deviceName) {
            throw Error('Invalid env device name!');
        }

        return deviceName;
    }
}
