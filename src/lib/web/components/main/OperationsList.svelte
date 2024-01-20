<script lang="ts">
    import Title from "../title/Title.svelte";
    import Operation from "./Operation.svelte";

	let search: string;
    
    type RecipeOperation = {
        name: string
        disabled: boolean,
        breakpoint: boolean
    }

    let operations: RecipeOperation[] = [
        {
            name: "To Base64",
            disabled: false,
            breakpoint: false,
        },
        {
            name: "From Base64",
            disabled: false,
            breakpoint: false
        }
    ];
    
</script>

<div class="operations">
    <Title title="Operations"/>
    <div class="SearchBar">
        <input bind:value={search} on:drop={(e) => e.preventDefault()} type="text" placeholder="Search..." />
    </div>
    <section class="operations-list">
        {#each operations as operation_name}
            <Operation name={operation_name.name} />
        {/each}
    </section>
</div>

<style>
    .SearchBar {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        width: 100%;
        height: calc(1.5em + 0.875rem + 2px);
        background-color: var(--primary-background-color);
        border-bottom: 1px solid var(--primary-border-color);
    }

    .SearchBar input {
        width: 100%;
        height: 100%;

        color: var(--primary-font-color);
        
        padding-left: 10px;
        padding-right: 10px;

        background-color: transparent;
        border: none;

        font-size: 1rem;
        font-weight: 400;
    }

</style>
