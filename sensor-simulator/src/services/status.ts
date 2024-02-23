import { IGenerator } from '../infra/generator';
import { IMessaging } from '../infra/messaging';

export interface IStatus {
    batteryVoltageGenerator: IGenerator;
    signalGenerator: IGenerator;
    messaging: IMessaging;
    device: string;
    send(): void;
}
