<script lang="ts">
    import type { BaseOperation } from "../../../core/baseOperation";
    import { operations } from "../../../core/runOperations";
    import RecipeOperation from "./RecipeOperation.svelte";

    let ingredients: [string, BaseOperation][] = [];
    operations.subscribe((value) => (ingredients = value));

    console.log(ingredients);
</script>

<ul class="recipe-list">
    {#each ingredients as [name, operation], id (id)}
        <RecipeOperation id={id.toString()} {name} {operation} />
    {/each}
</ul>

<style>
    .recipe-list {
        width: 100%;
        height: 100%;
    }
</style>
