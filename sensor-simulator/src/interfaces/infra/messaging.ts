import { AlgorithmData } from '../../entities/algorithm_data';
import { HealthyData } from '../../entities/healthy_data';

export interface IMessaging {
    publishData(topic: string, data: AlgorithmData): void;
    publishHealthy(topic: string, data: HealthyData): void;
}
