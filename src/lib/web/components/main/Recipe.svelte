<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import Title from "../title/Title.svelte";
    import OperationPreview from "./OperationPreview.svelte";
    import RecipeOperation from "./RecipeOperation.svelte";

    let icons = {
      "save": { description: "Save recipe" },
      "folder": { description: "Open recipe"},
      "delete": { description: "Delete recipe", func: () => baked_operations = []}
    }
    

    let baked_operations: string[] = [];
    let dragged: boolean = false;
    let dragged_name: string | undefined;

    async function run_me_daddy(operation_list: string[]) {
        let output = document.getElementById("output-textarea");
        let input = document.getElementById("input-textarea")?.value;

        console.log(operation_list)
        console.log(input)

        for (let i = 0; i < operation_list.length; i++) {
            if (operation_list[i] === "To Base64") {
                let request = {
                    input: input,
                    params: {}
                }
                console.log(`params: `, request)
                input = await invoke('to_base64', {request: JSON.stringify(request)})
            }
        }

        output.value = input;

    }

    $: run_me_daddy(baked_operations)

    function handleDragLeave(event: DragEvent) {
        event.preventDefault();
        dragged = false;
    }

    function handleDrop(event: DragEvent) {
        event.preventDefault();
        dragged = false;
        let new_operation = event.dataTransfer?.getData("text");
        if (new_operation?.length !== 0) {
            baked_operations = [...baked_operations, new_operation || ""]
        }
    }

    function handleDragOver(event: DragEvent) {
        event.preventDefault();
        dragged_name = event.dataTransfer?.getData('text');
        if (dragged_name?.length !== 0) {
            dragged = true;
        }
    }

</script>

<div class="recipe">
  <Title title="Recipe" id="recipe" {icons}/>
  <ul 
    on:drop={handleDrop}
    on:dragleave={handleDragLeave}
    on:dragover={handleDragOver}
    class="recipe-list"
    >

    {#each baked_operations as name, id}
        <RecipeOperation {id} {name}/>        
    {/each}

    {#if dragged}
        <OperationPreview name={dragged_name}/>
    {/if}

  </ul>
</div>

<style>
    .recipe-list {
        
        width: 100%;
        height: 100%;
    }
</style>
