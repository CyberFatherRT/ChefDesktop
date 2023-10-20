<script lang="ts">
    import Title from "../title/Title.svelte";

    let icons = {
        "save": "Save output to file",
        "content_copy": "Copy raw output- to the clipboard",
        "open_in_browser": "Replace input with output-",
        "fullscreen": "Maximase output- pane"
    };

</script>

<div class="output">
    <Title title="Output" id="output" {icons}/>

    <div class="output-wrapper">
        <div class="output-tabs"></div>
        <div class="output-text">
            <textarea id="output-textarea" readonly/>
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
