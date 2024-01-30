import { Modules, Operation, UserInputOptions, type Run } from "../baseOperation";
import { invoke } from "@tauri-apps/api";
import { KeyFormat, HashFunctions, OutputFormats } from "./Hmac";


export class Hmac extends Operation implements Run {
    constructor() {
        super();

        this.name = "HMAC";
        this.op_name = "hmac";
        this.module = Modules.Hashing;
        this.englishDescription = "Keyed-Hash Message Authentication Codes (HMAC) are a mechanism for message authentication using cryptographic hash functions.";
        this.russianDescription = "Keyed-Hash Message Authentication Codes (HMAC) — это механизм аутентификации сообщений с использованием криптографических хеш-функций.";
        this.infoURL = "https://wikipedia.org/wiki/HMAC";

        this.params = new Map([
            ["key", ""],
            ["key_format", ""],
            ["hash_function", ""],
            ["output_format", ""]
        ]);

        this.args = [
            {
                name: "Key",
                op_name: "key",
                type: UserInputOptions.inputWithType,
                value: KeyFormat,
                default_value: KeyFormat.UTF8,
                functions: {
                    "key": (input) => this.params.set('key', input)
                },
            },
            {
                name: "Hash function",
                op_name: "hash_function",
                type: UserInputOptions.enum,
                value: HashFunctions,
                default_value: HashFunctions.SHA256
            },
            {
                name: "Output format",
                op_name: "output_format",
                type: UserInputOptions.enum,
                value: OutputFormats,
                default_value: OutputFormats.Hex,
            }
        ];
    }

    async run(input: string): Promise<string> {
        let request = {
            input: input,
            params: {
                key: this.params.get('key') ?? "",
                key_format: this.params.get('key_format') ?? "",
                hash_function: this.params.get('hash_function') ?? "",
                output_format: this.params.get('output_format') ?? "",
            }
        };
        return await invoke(this.op_name, { request: JSON.stringify(request) }) as string;
    }

}

