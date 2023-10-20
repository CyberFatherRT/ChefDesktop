<script lang="ts">
  import Title from "../title/Title.svelte";

  let icons = {
    "save": "Save recipe",
    "folder": "Open recipe",
    "delete": "Delete recipe"
  }

</script>

<div class="recipe">
  <Title title="Recipe" id="recipe" {icons}/>
</div>
