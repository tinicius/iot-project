import { IGenerator } from '../infra/generator';
import { IMessaging } from '../infra/messaging';

export interface IStatus {
    batteryVoltageGenerator: IGenerator;
    signalGenerator: IGenerator;
    messaging: IMessaging;
    send(): void;
}
