import { IGenerator } from "../../interfaces/infra/generator";

export class RandomGenerator implements IGenerator {

    constructor(private min: number, private max: number) {

    }

    generate(): number {
        return (
            Math.random() * (this.max - this.min) +
            this.min
          );
    }
}