import { Service, ServiceType } from '../../entities/service';
import { IGenerator } from '../../infra/generator';
import { IMessaging } from '../../infra/messaging';
import { logger } from '../../utils/logger';
import { IService } from '../service';

export class HumidityService implements IService {
    generator: IGenerator;
    messaging: IMessaging;

    constructor(generator: IGenerator, messaging: IMessaging) {
        this.generator = generator;
        this.messaging = messaging;
    }

    send(): void {
        const value = this.generator.generate();
        const device = this.getDevice();

        if (!device) {
            logger.error('Invalid device name!');
            return;
        }

        const data: Service = {
            value,
            time: Date.now(),
            device,
            type: ServiceType.Humidity,
        };

        this.messaging.publishService(data);
    }

    private getDevice(): string | null {
        const deviceName = process.env.DEVICE;

        if (!deviceName) return null;

        return deviceName;
    }
}
