<script lang="ts">
    import Title from "../title/Title.svelte";
    import OperationPreview from "./OperationPreview.svelte";
    import RecipeOperation from "./RecipeOperation.svelte";

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

    function handleDragLeave(event: DragEvent) {
        event.preventDefault();
        dragged_over = false;
        dragged_leave = true;
        console.log(`dragged leave: ${dragged_leave}`);
    }

    function handleDrop(event: DragEvent) {
        event.preventDefault();
        dragged_over = false;
        let new_operation = event.dataTransfer?.getData("text");
        if (new_operation?.length !== 0) {
            baked_operations = [...baked_operations,  {name: new_operation || "", disabled: false, breakpoint: false}]
        }
        console.log(`dragged drop: ${dragged_leave}`);
    }

    function handleDragOver(event: DragEvent) {
        event.preventDefault();
        dragged_name = event.dataTransfer?.getData('text');
        dragged_over = true;
        dragged_leave = false;
        console.log(`dragged over: ${dragged_over}`);
    }

    window.addEventListener("dragover", (event: DragEvent) => {
        event.preventDefault();
        console.log("dragover");
    });

    function handleDragEnd(event: DragEvent) {
        event.preventDefault();
        let id = parseInt(event.target.id);
        if (dragged_leave) {
            baked_operations = baked_operations.filter((_, idx) => idx !== id);
        }
        console.log(`dragged end: ${dragged_leave}`);
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
