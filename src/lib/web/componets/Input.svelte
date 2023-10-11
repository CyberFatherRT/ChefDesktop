<script lang="ts">
  import Title from "./Title.svelte";

  let icons = {
    "add": "Add a new input tab",
    "folder_open": "Open folder as input",
    "input": "Open file as input",
    "delete": "Clear input and output",
    "view_compact": "Reset pane layout"
  }
</script>

<div class="input">
  <Title title="Input" id="input" {icons}/>
</div>


<style>
  .input {
    height: calc(50% - 2px);
  }
</style>
