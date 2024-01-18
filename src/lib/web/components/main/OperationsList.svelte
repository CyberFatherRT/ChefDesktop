<script lang="ts">
    import Title from "../title/Title.svelte";
    import Operation from "./Operation.svelte";

	let search: string;
    
    let operations: string[] = ["To Base64", "From Base64", "To Hex", "From Hex", "To Hexdump", "From Hexdump"];

</script>

<div class="operations">
    <Title title="Operations"/>
    <div class="SearchBar">
        <input bind:value={search} type="text" placeholder="Search..." />
    </div>
    <div class="operations-list">
        {#each operations as operation_name}
            <Operation name={operation_name} />
        {/each}
    </div>
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
