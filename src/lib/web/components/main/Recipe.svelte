<script lang="ts">
    import { Hmac } from "../../../core/operations/Hmac";
    import Title from "../title/Title.svelte";
    import RecipeOperation from "./RecipeOperation.svelte";

    let operation = new Hmac();

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
    let dragged_name: string;

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
        if (dragged_leave) {
            baked_operations = baked_operations.filter((_, idx) => idx !== parseInt(event.target.id));
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

    <!-- {#each baked_operations as name, idx} -->
    <!--     <RecipeOperation id={idx.toString()} name={name.name}/> -->
    <!-- {/each} -->
    <!--  -->
    <!-- {#if dragged_over &#38;&#38; dragged_name !== ""} -->
    <!--     <OperationPreview name={dragged_name}/> -->
    <!-- {/if} -->

    <RecipeOperation {operation}/>
  </ul>
</div>

<style>
    
    .recipe-list {
        width: 100%;
        height: 100%;
    }

</style>
