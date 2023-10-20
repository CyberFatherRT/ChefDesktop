<script lang="ts">
    import {invoke} from "@tauri-apps/api";
    import Title from "./Title.svelte";


    let icons = {
        "add": "Add a new input tab",
        "folder_open": "Open folder as input",
        "input": "Open file as input",
        "delete": "Clear input and output",
        "view_compact": "Reset pane layout"
    }

    let inputValue: string;

    async function foo() {
        let output = document.getElementById("output-textarea") as HTMLTextAreaElement;
        let request = JSON.stringify({
            input: inputValue,
            params: {
            }
        });
        output.innerText = await invoke("to_base64", {request});
    }

</script>

<div class="input">
    <Title title="Input" id="input" {icons}/>
    <div class="input-wrapper">
        <div class="input-tabs"></div>
        <div class="input-text">
            <textarea bind:value={inputValue} on:input={foo}/>
        </div>
        <div class="input-status-bar"></div>
    </div>
</div>


<style>
    .input {
        height: calc(50% - 2px);
    }

    .input-wrapper, .input-text {
        height: 100%;
        width: 100%;
    }

    textarea {
        width: 100%;
        resize: none;
        border-radius: 0;
        outline: none;
        border: none;
        height: calc(100% - 50px);
        background-color: transparent;
        color: white;
    }

</style>
