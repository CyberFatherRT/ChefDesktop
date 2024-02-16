<script lang="ts">
    import { get } from "svelte/store";
    import RecipeOperation from "./RecipeOperation.svelte";
    import { KuznechikEncrypt, KuznechikDecrypt, Hmac } from "../../../core/operations/Operations";
    import { BaseOperation } from "../../../core/baseOperation";
    import { operations } from "../../../core/runOperations";

    let foo = [
        new Hmac() as BaseOperation,
    ];

    operations.set(foo);

</script>

<ul class="recipe-list">
    {#each get(operations) as operation, id (id)}
        <RecipeOperation id={id.toString()} operation={operation}/>
    {/each}
</ul>

<style>

    .recipe-list {
        width: 100%;
        height: 100%;
    }

</style>
