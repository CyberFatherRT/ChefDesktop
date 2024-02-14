<script lang="ts">
    import { get } from "svelte/store";
    import RecipeOperation from "./RecipeOperation.svelte";
    import { KuznechikEncrypt, KuznechikDecrypt } from "../../../core/operations/Operations";
    import type { Operation } from "../../../core/baseOperation";
    import { operations } from "../../../core/runOperations";

    let foo = [
        new KuznechikEncrypt() as Operation,
        new KuznechikDecrypt() as Operation,
    ];

    operations.set(foo);

</script>

<ul class="recipe-list">
    {#each get(operations) as operation}
        <RecipeOperation operation={operation}/>
    {/each}
</ul>

<style>

    .recipe-list {
        width: 100%;
        height: 100%;
    }

</style>
