<script lang="ts">
    export let name: string;
</script>

<li draggable="true" class="operation">{name}</li>


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
