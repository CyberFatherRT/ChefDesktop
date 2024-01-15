<script lang="ts">
    import VGutter from "./lib/web/components/gutters/VGutter.svelte";
    import Operations from "./lib/web/components/main/OperationsList.svelte";
    import Recipe from "./lib/web/components/main/Recipe.svelte";
    import IO from "./lib/web/components/IO/IO.svelte";
</script>


<main>
    <div class="content-wrapper">
        <Operations/>
        <VGutter/>
        <Recipe/>
        <VGutter/>
        <IO/>
    </div>
</main>


<style>
    .content-wrapper {
        display: grid;
        grid-template-columns: calc(20% - 2px) 4px calc(30% - 4px) 4px calc(50% - 2px);
        height: 100%;
    }

    main {
        background-color: var(--primary-background-color);
    }
</style>
