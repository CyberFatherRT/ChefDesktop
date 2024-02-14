import { invoke } from "@tauri-apps/api";
import { Modules, UserInputOptions, type Operation } from "../baseOperation";
import { gsd } from "../runOperations";

export class KuznechikEncrypt implements Operation {

    name = "Kuznechik Encrypt";
    op_name = "kuznechik_encrypt";
    module = Modules["Encryption / Encoding"];
    englishDescription = "Kuznyechik is a symmetric block cipher. It has a block size of 128 bits and key length of 256 bits. It is defined in the National Standard of the Russian Federation GOST R 34.12-2015 and also in RFC 7801.";
    russianDescription = "Кузнечик — симметричный блочный шифр. Он имеет размер блока 128 бит и длину ключа 256 бит. Он определен в Национальном стандарте РФ ГОСТ Р 34.12-2015, а также в RFC 7801.";
    infoURL = "https://en.wikipedia.org/wiki/Kuznyechik";

    params = new Map([
        ["key", ""],
        ["key_format", Key_IV_Format.HEX],
        ["iv", ""],
        ["iv_format", Key_IV_Format.HEX],
        ["mode", Mode.CBC],
        ["padding", Padding.NO],
        ["input_format", InputFormat.Raw],
        ["output_format", OutputFormat.Hex],
    ]);


    is_disable = false;
    is_breakpoint = false;

    // @ts-ignore
    args = [
        {
            name: "Key",
            op_name: "key",
            type: UserInputOptions.inputWithType,
            value: Key_IV_Format,
            default_value: Key_IV_Format.HEX,
            functions: {
                input: (input: CustomEvent) => this.event_function(input, 'key'),
                enum: (input: CustomEvent) => this.event_function(input, 'key_format'),
            },
        },
        {
            name: "IV",
            op_name: "iv",
            type: UserInputOptions.inputWithType,
            value: Key_IV_Format,
            default_value: Key_IV_Format.HEX,
            functions: {
                input: (input: CustomEvent) => this.event_function(input, 'iv'),
                enum: (input: CustomEvent) => this.event_function(input, 'iv_format'),
            },
        },
        {
            name: "Input type",
            op_name: "input_format",
            type: UserInputOptions.enum,
            value: InputFormat,
            default: InputFormat.Raw,
            function: {
                enum: (input: CustomEvent) => this.event_function(input, "input_format"),
            }
        },
        {
            name: "Output type",
            op_name: "output_format",
            type: UserInputOptions.enum,
            value: InputFormat,
            default: InputFormat.Hex,
            function: {
                enum: (input: CustomEvent) => this.event_function(input, "output_format"),
            }
        },
        {
            name: "Block mode",
            op_name: "mode",
            type: UserInputOptions.enum,
            value: Mode,
            default_value: Mode.ECB,
            function: {
                enum: (input: CustomEvent) => this.event_function(input, "mode"),
            }
        },
        {
            name: "Padding",
            op_name: "padding",
            type: UserInputOptions.enum,
            value: Padding,
            defualt_value: Padding.PKCS5,
            function: {
                enum: (input: CustomEvent) => this.event_function(input, "padding"),
            }
        }
    ]

    event_function(input: CustomEvent, key: string) {
        this.params.set(key, input.detail.value)
        gsd()
    }

    async run(input: string): Promise<string> {
        let request = { input, params: Object.fromEntries(this.params) }
        console.log(request)
        return await invoke(this.op_name, { request: JSON.stringify(request) })
    }
}

enum OutputFormat {
    Hex = "hex",
    Base64 = "base64",
    Raw = "raw",
}

enum InputFormat {
    Hex = "hex",
    Base64 = "base64",
    Raw = "raw",
}

enum Key_IV_Format {
    HEX = "hex",
    UTF8 = "utf8",
    Latin1 = "latin1",
    Base64 = "base64",
}

enum Mode {
    CBC = "CBC",
    CTR = "CTR",
    OFB = "OFB",
    CFB = "CFB",
    ECB = "ECB",
}

enum Padding {
    NO = "no",
    PKCS5 = "pkcs5",
    Zero = "zero",
    Random = "random",
}
