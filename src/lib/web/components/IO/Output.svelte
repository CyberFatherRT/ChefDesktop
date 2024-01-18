<script lang="ts">
    import Title from "../title/Title.svelte";
    import { writeText } from "@tauri-apps/api/clipboard";

    let output: string;


    let icons = {
        "save": { description: "Save output to file" },
        "content_copy": { description: "Copy raw output- to the clipboard", func: () => writeText(output)},
        "open_in_browser": { description: "Replace input with output" },
        "fullscreen": { description: "Maximase output- pane" }
    };

</script>

<div class="output">
    <Title title="Output" id="output" {icons}/>

    <div class="output-wrapper">
        <div class="output-tabs"></div>
        <div class="output-text">
            <textarea bind:value={output} id="output-textarea" readonly/>
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
