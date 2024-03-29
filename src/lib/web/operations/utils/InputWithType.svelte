<script lang="ts">
	import type { Arg } from "../../../core/baseOperation";

	export let arg: Arg;

	let { name, default_value, value } = arg;

	let id = Math.random().toString(36).substring(4);
	let input: string = "";
	let selected: string = (default_value as string).toLowerCase();

	const getInput = () => arg.functions.input(input);
	const getEnumValue = () => arg.functions.enum(value[selected]);
</script>

<div class="container">
	<div class="input-field">
		<label for={id}>{name}</label>
		<input bind:value={input} on:input={getInput} {id} type="text" />
	</div>

	<div class="enum-feild">
		<select bind:value={selected} on:change={getEnumValue} id={`select-${id}`}>
			{#each Object.keys(value) as name}
				{#if value[name] == selected}
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
		border-top-left-radius: 4px;

		padding: 20px 12px 6px 12px !important;
		background-image: none;
		background-color: var(--arg-background);
		background-position: 100%, 100%;
		color: var(--arg-font-color);
		border: none;
		border-top-right-radius: 0 !important;
		height: 42px;

		text-align: center;
	}

	input:focus {
		background-color: var(--arg-background);
		background-image: linear-gradient(to top, #19d2ad 2px, rgba(25, 118, 210, 0) 2px), linear-gradient(to top, rgba(0, 0, 0, 0.26) 1px, rgba(0, 0, 0, 0) 1px);
		filter: brightness(100%);

		background-size:
			100% 100%,
			100% 100%;
		transition-duration: 0.3s;
		border-color: #9acffa;
		outline: 0;
	}

	.input-field {
		padding-top: 0;
		margin-bottom: 0;
		position: relative;
		text-align: left;
	}

	.container {
		display: flex;
		flex-flow: row wrap;
		justify-content: flex-start;
		margin-top: 1rem;
		flex-wrap: wrap;
		align-items: stretch;
	}

	.enum-feild {
		background-color: var(--arg-background);
		display: flex;
		border-top-left-radius: 0;
		border-top-right-radius: 4px;
		color: var(--rec-list-operation-font-color);
	}

	select {
		color: #6c757d;
		background-color: transparent;
		border: none;
		justify-items: center;
	}

	option {
		background-color: transparent;
	}
</style>
