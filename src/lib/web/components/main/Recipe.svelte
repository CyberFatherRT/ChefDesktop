<script lang="ts">
    import Title from "../title/Title.svelte";

    let icons = {
      "save": "Save recipe",
      "folder": "Open recipe",
      "delete": "Delete recipe"
    }

    let dragged: boolean = false;
    let dragged_name: string;

    function handleDrop(event) {
        event.preventDefault();
        console.log(`Dropped data: ${event.dataTransfer.getData("text/plain")}`);
    }

    function handleDragOver(event) {
        event.preventDefault();
        dragged = true;
        dragged_name = event.dataTransfer.getData("text/plain");
        console.log(`Dragged over: ${dragged_name}`);
    }

</script>

<div class="recipe">
  <Title title="Recipe" id="recipe" {icons}/>
  <ul class="recipe-list" on:drop={handleDrop} on:dragleave={() => dragged = false} on:dragover={handleDragOver}>
    {#if dragged}
       <li>{dragged_name}</li> 
    {/if}
  </ul>
</div>

<style>
    .recipe-list {
        
        width: 100%;
        height: 100%;
    }
</style>
