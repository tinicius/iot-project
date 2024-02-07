import { AlgorithmData } from '../../entities/algorithm_data';
import { IGenerator } from '../../interfaces/infra/generator';
import { IMessaging } from '../../interfaces/infra/messaging';
import { IAlgorithm } from '../../interfaces/services/algorithm';

export class HumidityService implements IAlgorithm {
    generator: IGenerator;
    messaging: IMessaging;

    constructor(generator: IGenerator, messaging: IMessaging) {
        this.generator = generator;
        this.messaging = messaging;
    }

    send(): void {
        const value = this.generator.generate();
        const device = this.getDevice();

        const data: AlgorithmData = {
            value,
            time: Date.now(),
            device,
            type: 'HUMIDITY',
        };

        const topic = `IoTProject/data/${data.device}/${data.type}`;

        this.messaging.publishData(topic, data);
    }

    private getDevice(): string {
        const deviceName = process.env.DEVICE;

        if (!deviceName) {
            throw Error('Invalid env device name!');
        }

        return deviceName;
    }
}
