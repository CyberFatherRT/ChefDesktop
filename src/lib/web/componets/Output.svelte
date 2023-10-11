<script lang="ts">
  import Title from "./Title.svelte";

  let icons = {
    "save": "Save output to file",
    "content_copy": "Copy raw output to the clipboard",
    "open_in_browser": "Replace input with output",
    "fullscreen": "Maximase output pane"
  };
</script>

<div class="output">
  <Title title="Output" id="output" {icons}/>
</div>
