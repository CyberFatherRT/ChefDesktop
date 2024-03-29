import { invoke } from "@tauri-apps/api/core";
import { BaseOperation, Modules } from "../baseOperation";

export class AnalyseHash extends BaseOperation {
	name = "Analyse Hash";
	op_name = "AnalyseHash";
	module = Modules.Hashing;
	// prettier-ignore
	englishDescription = "Tries to determine information about a given hash and suggests which algorithm may have been used to generate it based on its length.";
	// prettier-ignore
	russianDescription = "Пытается определить информацию о заданном хэше и предлагает, какой алгоритм мог быть использован для его генерации, исходя из его длины.";
	// prettier-ignore
	infoURL = "https://wikipedia.org/wiki/Comparison_of_cryptographic_hash_functions";

	params = new Map();

	is_disable = false;
	is_breakpoint = false;

	async run(input: string): Promise<string> {
		const request = { input, params: Object.fromEntries(this.params) };
		const [result, status] = await invoke(this.op_name, {
			request: JSON.stringify(request)
		})
			.then((ok) => [ok, true])
			.catch((err) => [err, false]);

		if (status === false) {
			return "Invalid Hash";
		}

		const [hash_length, byte_length, bit_length, ...hashes] = result.split(" ");
		return `Hash length: ${hash_length}
Byte length: ${byte_length}
Bit length: ${bit_length}

Based on the length, this hash could have been generated by one of the following hashing functions:
${hashes.join("\n")}`;
	}
}
