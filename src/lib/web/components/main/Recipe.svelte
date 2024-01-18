<script lang="ts">
  import Title from "../title/Title.svelte";

  let icons = {
    "save": "Save recipe",
    "folder": "Open recipe",
    "delete": "Delete recipe"
  }

    function handleDragStart(e: DragEvent) {
        console.log(`drag start`);
        console.log(e);
    }

</script>

<div class="recipe">
  <Title title="Recipe" id="recipe" {icons}/>
  <ul class="recipe-list" on:dragenter={handleDragStart}></ul>
</div>

<style>
    .recipe-list {
        
        width: 100%;
        height: 100%;
    }
</style>
