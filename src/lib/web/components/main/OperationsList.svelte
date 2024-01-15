<script lang="ts">
    import { onMount } from "svelte";
    import Title from "../title/Title.svelte";
    import SearchBar from "./SearchBar.svelte";

	let search: string;
	let users: any = [];

    $: visibleUsers = search ?
        users.filter(user => {
        return user.name.first.match(`${search}.*`) || user.name.last.match(`${search}.*`)
    }) : users;

</script>

<div class="operations">
    <Title title="Operations"/>
    <SearchBar {search}/>
    <div class="operations-list">
        {#each visibleUsers as user}
            <div class="placeholder">
                {user.name.first}
            </div>
        {/each}
    </div>
</div>

<style>
    .placeholder {
        height: 40px;
        background-color: white;
    }
</style>
