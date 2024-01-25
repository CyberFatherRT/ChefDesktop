import { invoke } from '@tauri-apps/api'
import { writable, type Writable } from "svelte/store"

export const input: Writable<string> = writable('');
export const output: Writable<string> = writable('');
export const operations: Writable<string[]> = writable([]);

input.subscribe(async (e) => output.set(await foo(e)))
operations.subscribe((e) => console.log(e))


export async function foo(input: string) {

    let request = [
        {
            name: "to_base64",
            request: JSON.stringify( { input: input, params: {} }),
            is_disable: false,
            breakpoint: false,
        },
    ];

    for (let item of request) {
        if (item.is_disable) {
            continue
        } else if (item.breakpoint) {
            break
        }
        input = await invoke(item.name, {request: item.request})
    }

    return input
}
