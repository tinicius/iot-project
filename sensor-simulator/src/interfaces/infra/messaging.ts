import { AlgorithmData } from "../../entities/algorithm_data";

export interface IMessaging {
    publish(data: AlgorithmData): void
}