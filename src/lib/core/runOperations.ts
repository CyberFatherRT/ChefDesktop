import { get, writable, type Writable } from "svelte/store"
import { BaseOperation } from "./baseOperation";

export const input: Writable<string> = writable('');
export const output: Writable<string> = writable('');
export const operations: Writable<BaseOperation[]> = writable([]);

input.subscribe(async () => await gsd())
operations.subscribe(async () => await gsd())

export async function gsd() {
    let local_input = get(input);

    for (let value of get(operations)) {
        if (value.is_disable) continue
        if (value.is_breakpoint) break
        local_input = await value.run(local_input)
    }

    output.set(local_input)
}
