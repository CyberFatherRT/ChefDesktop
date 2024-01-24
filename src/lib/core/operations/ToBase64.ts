import { Modules, Operation, type Run, UserInputOptions } from "../baseOperation";

export default class ToBase64 extends Operation implements Run {
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
    ];
  }
    run(request: object): string {
        throw new Error("Method not implemented.");
    }
}

enum Foo {
    'Standard (RFC 4648): A-Za-z0-9+/=' = 'A-Za-z0-9+/=',
    'URL safe (RFC 4648 5): A-Za-z0-9-_' = 'A-Za-z0-9-_',
    'Filename safe: A-Za-z0-9+-=' = 'A-Za-z0-9+\-=',
    'itoa64: ./0-9A-Za-z=' = './0-9A-Za-z=',
    'XML: A-Za-z0-9_.' = 'A-Za-z0-9_.',
    'y64: A-Za-z0-9._-' = 'A-Za-z0-9._-',
    'z64: 0-9a-zA-Z+/=' = '0-9a-zA-Z+/=',
    'Radix-64 (RFC 4880): 0-9A-Za-z+/=' = '0-9A-Za-z+/=',
    'Uuencoding: [space]-_' = ' -_',
    'Xxencoding: +-0-9A-Za-z' = '+\-0-9A-Za-z',
    'BinHex: !-,-0-689@A-NP-VX-Z[`a-fh-mp-r' = '!-,-0-689@A-NP-VX-Z[`a-fh-mp-r',
    'ROT13: N-ZA-Mn-za-m0-9+/=' = 'N-ZA-Mn-za-m0-9+/=',
    'UNIX crypt: ./0-9A-Za-z' = './0-9A-Za-z',
    'Atom128: /128GhIoPQROSTeUbADfgHijKLM+n0pFWXY456xyzB7=39VaqrstJklmNuZvwcdEC' = '/128GhIoPQROSTeUbADfgHijKLM+n0pFWXY456xyzB7=39VaqrstJklmNuZvwcdEC',
    'Megan35: 3GHIJKLMNOPQRSTUb=cdefghijklmnopWXYZ/12+406789VaqrstuvwxyzABCDEF5' = '3GHIJKLMNOPQRSTUb=cdefghijklmnopWXYZ/12+406789VaqrstuvwxyzABCDEF5',
    'Zong22: ZKj9n+yf0wDVX1s/5YbdxSo=ILaUpPBCHg8uvNO4klm6iJGhQ7eFrWczAMEq3RTt2' = 'ZKj9n+yf0wDVX1s/5YbdxSo=ILaUpPBCHg8uvNO4klm6iJGhQ7eFrWczAMEq3RTt2',
    'Hazz15: HNO4klm6ij9n+J2hyf0gzA8uvwDEq3X1Q7ZKeFrWcVTts/MRGYbdxSo=ILaUpPBC5' = 'HNO4klm6ij9n+J2hyf0gzA8uvwDEq3X1Q7ZKeFrWcVTts/MRGYbdxSo=ILaUpPBC5'
}


