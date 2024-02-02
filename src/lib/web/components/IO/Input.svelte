<script lang="ts">
    import Title from "../title/Title.svelte";
    import { input } from "../../../core/runOperations";

    let icons = {
        "add": { description: "Add a new input tab"},
        "folder_open": { description: "Open folder as input"},
        "input": { description: "Open file as input"},
        "delete": { description: "Clear input and output"},
        "view_compact": { description: "Reset pane layout"}
    }

    let inputValue: string = "";
    $: input.set(inputValue)

</script>

<div class="input">
    <Title title="Input" id="input" {icons}/>
    <div class="input-wrapper">
        <div class="input-tabs"></div>
        <div class="input-text">
            <textarea id="input-textarea" bind:value={inputValue} on:drop={(e) => e.preventDefault()}/>
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
