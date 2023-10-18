<script lang="ts">
  import { invoke } from "@tauri-apps/api";
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
    console.log(await invoke("greet", {name: inputValue}));
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
    border-radius: 0;
    border: none;
    height: 100%;
    background-color: transparent;
    color: white;
  }
</style>
