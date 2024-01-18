<script lang="ts">

    export let name: string;
    
    function handleDragStart(e: DragEvent) {
        e.dataTransfer?.setData("text", name);
    }

</script>

<li on:dragstart={handleDragStart} draggable="true" class="operation">
    {name}
</li>


<style>
    
    .operation {
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
