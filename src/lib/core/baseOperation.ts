import { invoke } from "@tauri-apps/api/core";
import { gsd } from "./runOperations";

export abstract class BaseOperation {
    name: string = "";
    op_name: string = "";
    module: Modules = Modules.Other;
    englishDescription: string = "";
    russianDescription: string = "";
    infoURL: string | null = null;
    args: Arg[] = [];
    params: Map<string, number | string | boolean> = new Map();
    is_disable: boolean = false;
    is_breakpoint: boolean = false;

    event_function(input: string, key: string) {
        this.params.set(key, input)
        gsd()
    }

    serialize() {
        return {
            name: this.op_name,
            request: JSON.stringify(Object.fromEntries(this.params))
        }
    }

    async run(input: string): Promise<string> {
        let request = { input, params: Object.fromEntries(this.params) }
        return await invoke(this.op_name, { request: JSON.stringify(request) })
    }
}

export interface Arg {
    name: string,
    op_name: string,
    type: UserInputOptions,
    value: any,
    default_value: string | number | boolean,
    functions: {
        [key: string]: (input: string) => void,
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

