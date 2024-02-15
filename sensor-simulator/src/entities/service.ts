export interface Service {
    device: string;
    type: ServiceType;
    value: number;
    time: number;
}

export enum ServiceType {
    Temperature = 0,
    Humidity = 1,
}
