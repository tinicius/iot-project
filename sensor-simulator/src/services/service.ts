import { IGenerator } from '../infra/generator';
import { IMessaging } from '../infra/messaging';

export interface IService {
    generator: IGenerator;
    messaging: IMessaging;
    send(): void;
}
