<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let name: string | undefined;
    export let id: string = "";

    const dispatch = createEventDispatcher();

    function on_leave() {
        dispatch('onleave', {id: id})
    }

    function on_drop() {
        dispatch('ondrop', {id: id})
    }

</script>

<li {id} on:dragleave={on_leave} on:dragend={on_drop} draggable="true" class="preview">{name}</li>

<style>

    .preview {
        color: var(--rec-list-operation-font-color);
        font-weight: var(--rec-list-operation-font-weight);

        font-family: var(--primary-font-family);
        font-size: var(--primary-font-size);
        line-height: var(--primary-line-height);
        
        padding: 14px;
        cursor: grab;
        position: relative;

        width: 100%;
        height: 49px;

        background-color: var(--rec-list-operation-bg-color);
        border-bottom: 1px solid var(--rec-list-operation-border-color);

        filter: brightness(93%);
    }
    
</style>
