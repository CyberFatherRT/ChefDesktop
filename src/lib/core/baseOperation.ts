export interface Run {
    name: string;
    op_name: string;
    module: Modules;
    englishDescription: string;
    russianDescription: string;
    infoURL: string | null;
    args: Arg[];
    params: Map<string, string>
    run(input: string): Promise<string>
}

export type Arg = {
    name: string,
    op_name: string,
    type: UserInputOptions,
    value: any,
    default_value: string | number | boolean,
    functions: {
        [key: string]: (input: CustomEvent) => void,
    },
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
