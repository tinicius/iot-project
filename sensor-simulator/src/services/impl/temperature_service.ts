import { Service, ServiceType } from '../../entities/service';
import { IGenerator } from '../../infra/generator';
import { IMessaging } from '../../infra/messaging';
import { logger } from '../../utils/logger';
import { IService } from '../service';

export class TemperatureService implements IService {
    generator: IGenerator;
    messaging: IMessaging;
    device: string;

    constructor(generator: IGenerator, messaging: IMessaging, device: string) {
        this.generator = generator;
        this.messaging = messaging;
        this.device = device;
    }

    send(): void {
        const value = this.generator.generate();
        const device = this.device;

        if (!device) {
            logger.error('Invalid device name!');
            return;
        }

        const data: Service = {
            value,
            time: Date.now(),
            device,
            type: ServiceType.Temperature,
        };

        this.messaging.publishService(data);
    }
}
