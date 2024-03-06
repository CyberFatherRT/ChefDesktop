import { get, writable, type Writable } from "svelte/store"
import { BaseOperation } from "./baseOperation";
import { invoke } from "@tauri-apps/api/tauri";

export const input: Writable<string> = writable('');
export const output: Writable<string> = writable('');
export const operations: Writable<[string, BaseOperation][]> = writable([]);

input.subscribe(async () => await gsd())
operations.subscribe(async () => await gsd())

export async function gsd() {

    let foo = get(operations).map(([_, op]) => op.serialize())

    await invoke("gsd", { input: get(input), ops: foo })
    .catch(err => console.log(err))


    // let local_input = get(input);
    //
    // for (let [_, op] of get(operations)) {
    //     if (op.is_disable) continue
    //     if (op.is_breakpoint) break
    //     local_input = await op.run(local_input)
    // }

    output.set(get(input))
}
