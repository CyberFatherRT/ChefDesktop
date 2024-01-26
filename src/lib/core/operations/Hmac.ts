import { Modules, Operation, UserInputOptions, type Run } from "../baseOperation";
import { invoke } from "@tauri-apps/api"; 

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
        ])

        this.args = [
            {
                name: "Key",
                op_name: "key",
                type: UserInputOptions.inputWithType,
                value: KeyFormat,
                default_value: KeyFormat.UTF8
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
        ]
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
        }
        return await invoke(this.op_name, { request: JSON.stringify(request) }) as string
    }

}

enum OutputFormats {
    Hex = "hex",
    Base64 = "base64",
    Raw = "raw",
}

enum HashFunctions {
    MD2 = "md2",
    MD4 = "md4",
    MD5 = "md5",
    SHA1 = "sha1",
    SHA224 = "sha224",
    SHA256 = "sha256",
    SHA384 = "sha384",
    SHA512 = "sha512",
    SHA512_224 = "sha512_224",
    SHA512_256 = "sha512_256",
    Ripemd128 = "ripemd128",
    Ripemd160 = "ripemd160",
    Ripemd256 = "ripemd256",
    Ripemd320 = "ripemd320",
    WhirlPool = "whirlpool",
}

enum KeyFormat {
    UTF8 = "utf8",
    Base64 = "base64",
    Hex = "hex",
    Binary = "binary",
    Latin1 = "latin1"
}

