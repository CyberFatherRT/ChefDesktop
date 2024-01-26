<script lang="ts">
    export let name: string | undefined;
    export let id: string = "";
</script>

<li {id} draggable="true" class="preview">
    <title>
        <p>{name}</p>
        <div class="recipe-icons">
            <button> <i class="material-icons">pause</i> </button>
            <button> <i class="material-icons">not_interested</i> </button>
        </div>
    </title>
    <slot/>
</li>

<style>

    title {
        display: flex;
        justify-content: space-between;
    }

    button {
        margin-right: 10px;

        background: transparent;
        border: none;
        color: var(--breakpoint-icon-color);

        float: right;
        cursor: pointer;
        font-size: 10px;

        line-height: var(--primary-line-height);
    }

    .recipe-icons {
        top: 13px;
        right: 10px;
    }

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
