import { invoke } from "@tauri-apps/api";
import { Modules, UserInputOptions, type Operation } from "../baseOperation";
import { gsd } from "../runOperations";

export class Hmac implements Operation {

    name = "HMAC";
    op_name = "hmac";
    module = Modules.Hashing;
    englishDescription = "Keyed-Hash Message Authentication Codes (HMAC) are a mechanism for message authentication using cryptographic hash functions.";
    russianDescription = "Keyed-Hash Message Authentication Codes (HMAC) — это механизм аутентификации сообщений с использованием криптографических хеш-функций.";
    infoURL = "https://wikipedia.org/wiki/HMAC";

    params = new Map([
        ["key", ""],
        ["key_format", KeyFormat.UTF8],
        ["hash_function", HashFunctions.SHA256],
        ["output_format", OutputFormats.Base64]
    ]);


    is_disable = false;
    is_breakpoint = false;

    // @ts-ignore
    args = [
        {
            name: "Key",
            op_name: "key",
            type: UserInputOptions.inputWithType,
            value: KeyFormat,
            default_value: KeyFormat.UTF8,
            functions: {
                input: (input: CustomEvent) => this.event_function(input, 'key'),
                enum: (input: CustomEvent) => this.event_function(input, 'key_format'),
            },
        },
        {
            name: "Hash function",
            op_name: "hash_function",
            type: UserInputOptions.enum,
            value: HashFunctions,
            default_value: HashFunctions.SHA256,
            functions: {
                enum: (input: CustomEvent) => this.event_function(input, 'hash_function'),
            }
        },
        {
            name: "Output format",
            op_name: "output_format",
            type: UserInputOptions.enum,
            value: OutputFormats,
            default_value: OutputFormats.Hex,
            functions: {
                enum: (input: CustomEvent) => this.event_function(input, 'output_format'),
            }
        }
    ]

    event_function(input: CustomEvent, key: string) {
        this.params.set(key, input.detail.value)
        gsd()
    }

    async run(input: string): Promise<string> {
        let request = {
            input,
            params: Object.fromEntries(this.params)
        }
        return await invoke(this.op_name, { request: JSON.stringify(request) })
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
}

