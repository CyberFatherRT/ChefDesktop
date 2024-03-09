import { get, writable, type Writable } from "svelte/store";
import { BaseOperation } from "./baseOperation";
import { invoke } from "@tauri-apps/api/core";

export const input: Writable<string> = writable("");
export const output: Writable<string> = writable("");
export const operations: Writable<[string, BaseOperation][]> = writable([]);

input.subscribe(async () => await gsd());
operations.subscribe(async () => await gsd());

export async function gsd() {
    const foo = get(operations).map(([_, op]) => op.serialize());
    const result = await invoke("gsd", { input: get(input), ops: foo });

    output.set(result as string);
}
