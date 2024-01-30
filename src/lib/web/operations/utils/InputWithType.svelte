<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let placeholder: string;
    export let type_enum: any;
    export let enum_default: string;
    export let id: string;

    let input: string = "";
    let selected_enum_value: string = "";

    const dispatch = createEventDispatcher();
    
    const getInput = () => {
        dispatch('getInput', {
            value: input
        })
    }

    function getEnumValue() {
        dispatch('getEnumValue', {
            value: selected_enum_value
        })
    }

</script>

<div class="container">

    <div class="input-field">
        <label for={id}>{placeholder}</label>
        <input bind:value={input} on:input={getInput} {id} type="text">
    </div>
    
    <div class="enum-feild">
       <select bind:value={selected_enum_value} on:change={getEnumValue}>
            {#each Object.keys(type_enum) as name}
                {#if name == enum_default}
                    <option value={name} selected>{name}</option>
                {:else}
                    <option value={name}>{name}</option>
                {/if}
            {/each}
       </select> 
    </div>

</div>

<style>

    label {
        will-change: left, top, contents;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        width: calc(100% - 13px);

        top: 13px !important;
        left: 12px;
        z-index: 10;

        position: absolute;
        pointer-events: none;
        transition: 0.3s ease all;

        color: var(--arg-label-color);
        font-size: 1rem;
    }

    label:focus {
        animation: checkbox-off 0.3 forwards;
    }

    input {
        height: 100%;
        border-top-left-radius: 4px;

        padding: 20px 12px 6px 12px !important;
        background-image: none;
        background-color: var(--arg-background);
        background-position: 100%, 100%;
        color: var(--arg-font-color);
        border: none;
        border-top-right-radius: 0 !important;
        height: 42px !important;
        
        text-align: center;
    }

    input:focus {
        background-color: var(--arg-background);
        background-image: linear-gradient(to top, #19d2ad 2px, rgba(25, 118, 210, 0) 2px), linear-gradient(to top, rgba(0, 0, 0, 0.26) 1px, rgba(0, 0, 0, 0) 1px);
        filter: brightness(100%);

        background-size: 100% 100%, 100% 100%;
        transition-duration: 0.3s;
        border-color: #9acffa;
        outline: 0;
    }

    .input-field {
        margin-top: 1rem;
        padding-top: 0;

        flex: 3 200px;
        margin-bottom: 0;
        position: relative;
        display: flex;
        flex-wrap: wrap;
        align-items: stretch;
        width: 100%;
        text-align: left;
    }

    .container {
        display: flex;
        flex-flow: row wrap;
        justify-content: flex-start;
        column-gap: 14px;
        row-gap: 0;
    }

</style>
