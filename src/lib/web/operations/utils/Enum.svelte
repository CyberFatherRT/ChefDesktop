<script lang="ts">
	import type { Arg } from "../../../core/baseOperation";

	export let arg: Arg;

	let { value, default_value } = arg;

	let id = Math.random().toString(36).substring(4);
	let selected = default_value as string;

	const getEnumValue = () => arg.functions.enum(value[selected]);
</script>

<div class="container">
	<select bind:value={selected} on:change={getEnumValue} name="enum" id={`select-${id}`}>
		{#each Object.keys(value) as name}
			{#if value[name] == selected}
				<option value={name} selected>{name}</option>
			{:else}
				<option value={name}>{name}</option>
			{/if}
		{/each}
	</select>
</div>
