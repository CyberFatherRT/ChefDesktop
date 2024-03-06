import { get, writable, type Writable } from "svelte/store"
import { BaseOperation } from "./baseOperation";

export const input: Writable<string> = writable('');
export const output: Writable<string> = writable('');
// @ts-ignore
export const operations: Writable<[string, BaseOperation][]> = writable([]);

input.subscribe(async () => await gsd())
operations.subscribe(async () => await gsd())

export async function gsd() {

    let foo = get(operations)

    if (foo.length !== 0) {
        console.log(foo[0])
        console.log(JSON.stringify(foo[0]))
    }

    // let local_input = get(input);
    //
    // for (let [_, op] of get(operations)) {
    //     if (op.is_disable) continue
    //     if (op.is_breakpoint) break
    //     local_input = await op.run(local_input)
    // }

    output.set(get(input))
}
