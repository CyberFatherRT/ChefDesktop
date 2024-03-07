import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog"
import { input, output } from "../../../core/runOperations";
import { get } from "svelte/store";

export async function readFromFile() {

    const selectedFile = await open({
        multiple: false
    });

    // @ts-ignore
    let filePath = selectedFile.message.path;

    let content = await invoke("read_from_file", { path: filePath }) as string
    input.set(content)

}

export async function saveToFile() {
    const savePath = await save();
    let output_content = get(output);

    await invoke("save_to_file", { path: savePath, content: output_content })
}
