import { IGenerator } from "../infra/generator";
import { IMessaging } from "../infra/messaging";

export interface IHealthy {
    batteryGenerator: IGenerator;
    messaging: IMessaging;
    send(): void;
}