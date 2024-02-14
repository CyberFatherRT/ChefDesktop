<script lang="ts">
    import { UserInputOptions, type Operation } from "../../../core/baseOperation";
    import InputWithType from "../../operations/utils/InputWithType.svelte";

    export let operation: Operation;
    export let id: string = "";

    function disable() {
        operation.is_disable != operation.is_disable
    }

    function breakpoint() {
        operation.is_breakpoint != operation.is_breakpoint
    }

</script>

<li {id} draggable="true" class="preview">

    <title>
        <p>{operation.name}</p>
        <div>
            <button on:click={disable}> <i class="material-icons">pause</i> </button>
            <button on:click={breakpoint}> <i class="material-icons">not_interested</i> </button>
        </div>
    </title>

    {#each operation.args as arg}
        {#if arg.type === UserInputOptions.inputWithType}
        <InputWithType id="4"
                on:getInput={arg.functions.input}
                on:getEnumValue={arg.functions.enum}
                type_enum={arg.value}
                enum_default={arg.default_value}
                placeholder={arg.name}
            />
        {/if}
    {/each}

</li>

<style>

    title {
        display: flex;
        justify-content: space-between;
        height: 39px;
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
