import { IGenerator } from "../infra/generator";

export interface IHealthy {
    batteryGenerator: IGenerator;
    publish(): void;
}