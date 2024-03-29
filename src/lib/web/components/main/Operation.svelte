<script lang="ts">
	import { operationList } from "../../../core/operations/Operations";
	import { operations } from "../../../core/runOperations";

	export let name: string;

	function addOperation() {
		let random_key = Math.random().toString(36).substring(4);
		// @ts-ignore
		let operation = operationList[name];
		operations.update((ops) => [...ops, [random_key, new operation()]]);
	}
</script>

<button on:click={addOperation} class="operation">
	{name}
</button>

<style>
	.operation {
		cursor: grub;
		padding: 10px;
		position: relative;

		width: 100%;
		height: 41;

		color: var(--op-list-operation-font-color);
		background-color: var(--op-list-operation-bg-color);
		border: none;
		border-bottom: 1px solid var(--op-list-operation-border-color);

		list-style-type: none;
	}

	.operation:hover {
		filter: brightness(98%);
	}
</style>
