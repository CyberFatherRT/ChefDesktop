export abstract class Operation {
    name: string;
    op_name: string;
    module: Modules;
    englishDescription: string;
    russianDescription: string;
    infoURL: string | null;
    args: Arg[]

    constructor() {
        this.name = "";
        this.op_name = "";
        this.module = Modules.Other;
        this.englishDescription = "";
        this.russianDescription = "";
        this.infoURL = null;

        this.args = [];
    }
}

export interface Run {
    run(request: object): string
}

export type Arg = {
    name: string,
    op_name: string;
    type: UserInputOptions,
    value: any,
    default_value: string | number | boolean,
}

export enum Modules {
    "Data Format",
    'Encryption / Encoding',
    "Public Key",
    "Arithmetic / Logic",
    "Networking",
    "Language",
    "Utils",
    "Data / Time",
    "Extractors",
    "Compression",
    "Hashing",
    "Code tidy",
    "Forensics",
    "Multimedia",
    "Other",
    "Flow control",
}

export enum UserInputOptions {
    enum,
    enumWithValue,
    input,
    inputWithType,
    textarea,
    checkbox,
    intCounter,
}
