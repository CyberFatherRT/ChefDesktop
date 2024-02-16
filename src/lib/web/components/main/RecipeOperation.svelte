<script lang="ts">
    import { UserInputOptions, BaseOperation } from "../../../core/baseOperation";
    import Enum from "../../operations/utils/Enum.svelte";
    import InputWithType from "../../operations/utils/InputWithType.svelte";

    export let operation: BaseOperation;
    export let id: string = "";

    const disable = () => operation.is_disable != operation.is_disable;
    const breakpoint = () => operation.is_breakpoint != operation.is_breakpoint;

</script>

<li {id} draggable="true" class="preview">

    <title>
        <p>{operation.name}</p>
        <div>
            <button on:click={disable}> <i class="material-icons">pause</i> </button>
            <button on:click={breakpoint}> <i class="material-icons">not_interested</i> </button>
        </div>
    </title>

    <div class="arg-wrapper">
        {#each operation.args as arg}
            {#if arg.type === UserInputOptions.inputWithType}
                <InputWithType {arg}/>
            {:else if arg.type === UserInputOptions.enum}
                <Enum {arg}/>
            {/if}
        {/each}
    </div>

</li>

<style>

    title {
        display: flex;
        justify-content: space-between;
        height: 39px;
    }

    .arg-wrapper {
        display: flex;
        flex: row;
    }

    i {
        font-size: 18px;
    }

    p {
        font-size: 14px;
        color: var(--rec-list-operation-font-color);
        font-weight: var(--rec-list-operation-font-weight);
    }

    button {
        margin-right: 10px;

        background: transparent;
        border: none;
        color: var(--breakpoint-icon-color);

        float: right;
        cursor: pointer;

        line-height: var(--primary-line-height);
    }

    .preview {

        padding: 14px;
        cursor: grab;
        position: relative;

        width: 100%;

        background-color: var(--rec-list-operation-bg-color);
        border-bottom: 1px solid var(--rec-list-operation-border-color);

        filter: brightness(93%);
    }

</style>
