<script lang="ts">
    import { saveToFile } from "./io_utils";
    import Title from "../title/Title.svelte";
    import { writeText } from "@tauri-apps/api/clipboard";
    import { gsd, input, output } from "../../../core/runOperations";

    let icons = {
        "save": { description: "Save output to file", func: saveToFile },
        "content_copy": { description: "Copy raw output- to the clipboard", func: () => writeText(outputValue)},
        "open_in_browser": { description: "Replace input with output", func: () => { input.set(outputValue); gsd() }},
    };

    let outputValue: string;
    output.subscribe((value) => outputValue = value);

</script>

<div class="output">
    <Title title="Output" id="output" {icons}/>
    <div class="output-wrapper">
        <div class="output-tabs"></div>
        <div class="output-text">
            <textarea bind:value={outputValue} id="output-textarea" readonly/>
        </div>
        <div class="output-status-bar"></div>
    </div>
</div>

<style>

    .output {
        height: calc(50% - 2px);
    }

    .output-wrapper, .output-text {
        height: 100%;
        width: 100%;
    }

    textarea {
        width: 100%;
        resize: none;
        border-radius: 0;
        border: none;
        height: calc(100% - 48px);
        background-color: transparent;
        color: white;
    }

</style>
