<script lang="ts">
	import { tooltip } from "./title";

	export let title: string;
	export let icons: object = {};
	export let id: string = "";
</script>

<div class="title no-select">
	<p>{title}</p>
	<span>
		{#each Object.entries(icons) as [icon, values]}
			<button use:tooltip={{ content: values.description }} on:click={values.func} id="{id}_{icon}" tabindex="-1">
				<i class="material-icons">{icon}</i>
			</button>
		{/each}
	</span>
</div>

<style>
	.title {
		display: flex;
		justify-content: space-between;

		padding: 8px 12px;
		text-align: left;

		height: 48px;
		width: 100%;

		font-weight: var(--title-weight);
		font-size: var(--title-size);
		color: var(--title-color);
		line-height: calc(var(--title-height) - 14px);
		font-family: var(--primary-font-family);

		border-bottom: 1px solid var(--primary-border-color);
		background-color: var(--title-background-color);
	}

	p {
		font-size: 1.1rem;
		line-height: 1;
		display: inline-block;
		margin-top: 0.5rem;
	}

	button {
		margin-left: 2px;
		width: 2rem;
		min-width: 2rem;

		padding: 0;

		line-height: 0;
		border-radius: 50%;

		background-color: transparent;
		border: none;
		color: #fff;

		cursor: pointer;
	}

	button:hover {
		background-color: #484848;
		border-color: #484848;
		color: #3dba9f;

		transition: background-color 0.2s;
	}

	span {
		display: flex;
		flex-direction: row;

		right: 8px;
		top: 8px;
	}
</style>
