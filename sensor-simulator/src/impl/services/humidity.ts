import { AlgorithmData } from '../../entities/algorithm_data';
import { IGenerator } from '../../interfaces/infra/generator';
import { IMessaging } from '../../interfaces/infra/messaging';
import { IAlgorithm } from '../../interfaces/services/algorithm';
import * as os from 'os';

export class HumidityService implements IAlgorithm {
    generator: IGenerator;
    messaging: IMessaging;

    constructor(generator: IGenerator, messaging: IMessaging) {
        this.generator = generator;
        this.messaging = messaging;
    }

    send(): void {
        const value = this.generator.generate();

        const data: AlgorithmData = {
            value,
            time: Date.now(),
            device: os.userInfo().username,
            type: 'HUMIDITY',
        };

        const topic = `IoTProject/data/${data.device}/${data.type}`;

        this.messaging.publishData(topic, data);
    }
}
