import {Modules, Operation, type Run, UserInputOptions} from "../baseOperation";
import {invoke} from "@tauri-apps/api";

export default class ToBase64 extends Operation implements Run<Request, Promise<[Ok, boolean] | [Err, boolean]>> {
    constructor() {
        super();

        this.name = "To Base64";
        this.module = Modules["Encryption / Encoding"];
        this.englishDescription = "Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers.<br><br>This operation encodes raw data into an ASCII Base64 string.<br><br>e.g. <code>hello</code> becomes <code>aGVsbG8=</code>";
        this.russianDescription = "Base64 — это нотация для кодирования произвольных байтовых данных с использованием ограниченного набора символов, которые могут удобно использоваться людьми и обрабатываться компьютерами.<br><br>Эта операция декодирует необработанные данные в строку ASCII Base64.<br><br>* <code>привет</code> становится <code>0L/RgNC40LLQtdGCCg==</code>";
        this.infoURL = "https://wikipedia.org/wiki/Base64";

        this.args = [
            {
                name: "Alphabet",
                type: UserInputOptions.enumWithValue,
                value: Foo,
                default_value: Object.values(Foo)[0],
            }
        ]
    }

    async run(request: Request): Promise<[Ok, boolean] | [Err, boolean]> {
        let foo = await invoke("to_base64", {request: JSON.stringify(request)})
            .then(ok => (ok as Ok, true))
            .catch((err): [Err, boolean] => [err as Err, false]);
        return foo
    }
}

type Request = {
    input: string,
    params: {}
}

type Ok = {
    ok: string
}

type Err = {
    err: string
}

