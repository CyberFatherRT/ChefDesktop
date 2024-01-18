<script lang="ts">
    import { setContext } from "svelte";

    export let name: string;
    
    function handleDragStart(e) {
        e.dataTransfer.setData("text/plain", name);
    }

    function handleDrop(e) {
        e.preventDefault();
        setContext("recipe", e.dataTransfer.getData("text/plain"));
    }

</script>

<li on:dragstart={handleDragStart} on:drop={handleDrop} draggable="true" class="operation">
    {name}
</li>


<style>
    
    .operation {
        cursor: -webkit-grub;
        cursor: grub;
        padding: 10px;
        position: relative;

        width: 100%;
        height: 41;

        color: var(--op-list-operation-font-color);
        background-color: var(--op-list-operation-bg-color);
        border-bottom: 1px solid var(--op-list-operation-border-color);

        list-style-type: none;
    }

    .operation:hover {
        filter: brightness(98%);
    }

</style>
