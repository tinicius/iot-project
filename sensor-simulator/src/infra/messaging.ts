import { Service } from '../entities/service';
import { Status } from '../entities/status';

export interface IMessaging {
    publishService(data: Service): void;
    publishStatus(data: Status): void;
}
