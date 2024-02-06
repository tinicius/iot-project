import { IGenerator } from "../infra/generator";

export interface IData {
    generator: IGenerator;
    publish(): void;
}