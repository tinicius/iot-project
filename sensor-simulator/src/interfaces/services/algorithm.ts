import { IGenerator } from "../infra/generator";
import { IMessaging } from "../infra/messaging";

export interface IAlgorithm
{
    generator: IGenerator;
    messaging: IMessaging;
    send(): void;
}