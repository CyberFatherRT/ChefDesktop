<script lang="ts">
    import Title from "../title/Title.svelte";
    import OperationPreview from "./OperationPreview.svelte";
    import RecipeOperation from "./RecipeOperation.svelte";
    import { foo } from "../../../core/runOperations";

    let icons = {
      "save": { description: "Save recipe" },
      "folder": { description: "Open recipe"},
      "delete": { description: "Delete recipe", func: () => baked_operations = []}
    }

    type RecipeOperation = {
        name: string
        disabled: boolean,
        breakpoint: boolean
    }
    
    let baked_operations: RecipeOperation[] = [];
    let dragged_over: boolean = false;
    let dragged_leave: boolean = false;
    let dragged_name: string | undefined;

    $: foo(baked_operations)

    function handleDragLeave(event: DragEvent) {
        event.preventDefault();
        dragged_over = false;
        dragged_leave = true;
    }

    function handleDrop(event: DragEvent) {
        event.preventDefault();
        dragged_over = false;
        let new_operation = event.dataTransfer?.getData("text");
        if (new_operation?.length !== 0) {
            baked_operations = [...baked_operations,  {name: new_operation || "", disabled: false, breakpoint: false}]
        }
    }

    function handleDragOver(event: DragEvent) {
        event.preventDefault();
        dragged_name = event.dataTransfer?.getData('text');
        dragged_over = true;
        dragged_leave = false;
    }

    function handleDragEnd(event: DragEvent) {
        event.preventDefault();
        let id = parseInt(event.target.id);
        if (dragged_leave) {
            baked_operations = baked_operations.filter((_, idx) => idx !== id);
        }
    }

</script>

<div class="recipe">
  <Title title="Recipe" id="recipe" {icons}/>
  <ul 
    on:drop={handleDrop}
    on:dragleave={handleDragLeave}
    on:dragover={handleDragOver}
    on:dragend={handleDragEnd}
    class="recipe-list"
    >

    {#each baked_operations as name, idx}
        <RecipeOperation id={idx.toString()} name={name.name}/>
    {/each}

    {#if dragged_over && dragged_name !== ""}
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
