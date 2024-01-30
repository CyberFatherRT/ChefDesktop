<script lang="ts">
    import InputWithType from "./utils/InputWithType.svelte";
    import { Hmac } from "../../core/operations/Hmac";
    import { UserInputOptions } from "../../core/baseOperation";

    let operation = new Hmac();
</script>

{#each operation.args as arg}
    {#if arg.type === UserInputOptions.inputWithType}
        <InputWithType 
            id="4"
            on:getInput={arg.functions.input}
            on:getEnumValue={arg.functions.enum}
            type_enum={arg.value}
            placeholder={arg.name}
        />
    {/if}
{/each}
