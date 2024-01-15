<script>
     export let search = "";
</script>

<div class="SearchBar">
    <input bind:value={search} type="text" placeholder="Search..." />
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
