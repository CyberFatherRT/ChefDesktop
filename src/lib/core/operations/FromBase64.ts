import { BaseOperation, Modules, UserInputOptions } from "../baseOperation";

export class FromBase64 extends BaseOperation {

    name = "From Base64";
    op_name = "FromBase64";
    module = Modules["Encryption / Encoding"];
    englishDescription = "Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation decodes raw data into an ASCII Base64 string.";
    russianDescription = "Base64 — это нотация для кодирования произвольных байтовых данных с использованием ограниченного набора символов, которые могут удобно использоваться людьми и обрабатываться компьютерами.<br><br>Эта операция декодирует необработанные данные в строку ASCII Base64.";
    infoURL = "https://wikipedia.org/wiki/Base64";

    params = new Map([
        ["alphabet", "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"],
        ["remove_non_alphabetic_chars", "false"],
        ["strict_mode", "false"],
    ]);

    is_disable = false;
    is_breakpoint = false;

    // @ts-ignore
    args = [
        {
            name: "Alphabet",
            op_name: "alphabet",
            type: UserInputOptions.enumWithValue,
            value: Alphabet,
            default_value: Alphabet["Standard (RFC 4648): A-Za-z0-9+/="],
            functions: {
                enum: (input: string) => this.event_function(input, "alphabet"),
            },
        },
        {
            name: "Remove non-alphabet chars",
            op_name: "remove_non_alphabet_chars",
            type: UserInputOptions.checkbox,
            value: false,
            default_value: false,
            functions: {
                checkbox: (input: string) => this.event_function(input, "remove_non_alphabetic_chars"),
            },
        },
        {
            name: "Strict mode",
            op_name: "strict_mode",
            type: UserInputOptions.checkbox,
            value: false,
            default_value: false,
            functions: {
                checkbox: (input: string) => this.event_function(input, "strict_mode"),
            },
        }
    ]

}

enum Alphabet {
    "Standard (RFC 4648): A-Za-z0-9+/=" = "A-Za-z0-9+/=",
    "URL safe (RFC 4648 \u00A75): A-Za-z0-9-_" = "A-Za-z0-9-_",
    "Filename safe: A-Za-z0-9+-=" = "A-Za-z0-9+\\-=",
    "itoa64: ./0-9A-Za-z=" = "./0-9A-Za-z=",
    "XML: A-Za-z0-9_." = "A-Za-z0-9_.",
    "y64: A-Za-z0-9._-" = "A-Za-z0-9._-",
    "z64: 0-9a-zA-Z+/=" = "0-9a-zA-Z+/=",
    "Radix-64 (RFC 4880): 0-9A-Za-z+/=" = "0-9A-Za-z+/=",
    "Uuencoding: [space]-_" = " -_",
    "Xxencoding: +-0-9A-Za-z" = "+\\-0-9A-Za-z",
    "BinHex: !-,-0-689@A-NP-VX-Z[`a-fh-mp-r" = "!-,-0-689@A-NP-VX-Z[`a-fh-mp-r",
    "ROT13: N-ZA-Mn-za-m0-9+/=" = "N-ZA-Mn-za-m0-9+/=",
    "UNIX crypt: ./0-9A-Za-z" = "./0-9A-Za-z",
    "Atom128: /128GhIoPQROSTeUbADfgHijKLM+n0pFWXY456xyzB7=39VaqrstJklmNuZvwcdEC" = "/128GhIoPQROSTeUbADfgHijKLM+n0pFWXY456xyzB7=39VaqrstJklmNuZvwcdEC",
    "Megan35: 3GHIJKLMNOPQRSTUb=cdefghijklmnopWXYZ/12+406789VaqrstuvwxyzABCDEF5" = "3GHIJKLMNOPQRSTUb=cdefghijklmnopWXYZ/12+406789VaqrstuvwxyzABCDEF5",
    "Zong22: ZKj9n+yf0wDVX1s/5YbdxSo=ILaUpPBCHg8uvNO4klm6iJGhQ7eFrWczAMEq3RTt2" = "ZKj9n+yf0wDVX1s/5YbdxSo=ILaUpPBCHg8uvNO4klm6iJGhQ7eFrWczAMEq3RTt2",
    "Hazz15: HNO4klm6ij9n+J2hyf0gzA8uvwDEq3X1Q7ZKeFrWcVTts/MRGYbdxSo=ILaUpPBC5" = "HNO4klm6ij9n+J2hyf0gzA8uvwDEq3X1Q7ZKeFrWcVTts/MRGYbdxSo=ILaUpPBC5",
}
