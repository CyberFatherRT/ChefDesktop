import { get, writable, type Writable } from "svelte/store"
import type { Run } from "./baseOperation";

type Recipe = Map<string, Run>;
export const input: Writable<string> = writable('');
export const output: Writable<string> = writable('');
export const operations: Writable<Recipe> = writable();

input.subscribe(async (value) => output.set(await gtsd(value)))
operations.subscribe(async () => output.set(await gtsd(get(input))))

export async function gtsd(input: string) {
    // for (let item of get(operations).values()) {
    //     input = await item.run(input)
    // }
    return input
}
