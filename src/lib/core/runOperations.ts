import { get, writable, type Writable } from "svelte/store"
import { type Run } from "./baseOperation";

type RecipeItem = {
    is_disable: boolean,
    is_breakpoint: boolean,
    operation: Run,
}

type Recipe = Map<string, RecipeItem>

export const input: Writable<string> = writable('');
export const output: Writable<string> = writable('');
export const operations: Writable<Recipe> = writable(new Map());

input.subscribe(async () => await gsd())
operations.subscribe(async () => await gsd())

export async function gsd() {
    let local_input = get(input);

    for (let [_, value] of get(operations)) {
        if (value.is_disable) continue
        if (value.is_breakpoint) break
        local_input = await value.operation.run(local_input)
    }

    output.set(local_input)
}
