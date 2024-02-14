<script lang="ts">
    import Title from "../title/Title.svelte";
    import RecipeList from "../main/RecipeList.svelte";
    import { operations } from "../../../core/runOperations";

    let icons = {
      "save": { description: "Save recipe" },
      "folder": { description: "Open recipe"},
      "delete": { description: "Delete recipe", func: () => operations.set([])}
    }

</script>

<div class="recipe">
    <Title title="Recipe" id="recipe" {icons}/>
    <RecipeList/>
</div>

