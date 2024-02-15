import { Status } from '../../entities/status';
import { IGenerator } from '../../infra/generator';
import { IMessaging } from '../../infra/messaging';
import { logger } from '../../utils/logger';
import { IStatus } from '../status';

export class StatusService implements IStatus {
    constructor(
        public batteryVoltageGenerator: IGenerator,
        public signalGenerator: IGenerator,
        public messaging: IMessaging
    ) {}

    send(): void {
        const batteryVoltage = this.batteryVoltageGenerator.generate();
        const signal = this.signalGenerator.generate();
        const device = this.getDevice();

        if (!device) {
            logger.error('Invalid device name!');
            return;
        }

        const data: Status = {
            time: Date.now(),
            device,
            batteryVoltage,
            signal,
        };

        this.messaging.publishStatus(data);
    }

    private getDevice(): string | null {
        const deviceName = process.env.DEVICE;

        if (!deviceName) return null;

        return deviceName;
    }
}
