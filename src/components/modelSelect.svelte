<script lang="ts">
    import type { Model } from "@/lib/model";
    // import Icon from "@iconify/svelte";

    interface Props {
        choices: Model[];
        selected: Model;
    }

    let { choices, selected = $bindable() }: Props = $props();
    let repoId2model = Object.fromEntries(
        choices.map((model) => [model.repoId, model]),
    );

    const onSelect = (event: Event) => {
        event.preventDefault();

        const target = event.target as HTMLSelectElement;
        selected = repoId2model[target.value];
        console.log($state.snapshot(selected));
    };
</script>

<div class="p-2">
    <label
        for="model-select"
        class="block text-sm mb-1 text-gray-900 dark:text-white">Model</label
    >
    <select
        bind:value={selected.repoId}
        id="model-select"
        class="text-lg w-full pl-2 pr-8 py-2 rounded-sm border border-blue-300 dark:border-blue-600 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
        onchange={onSelect}
    >
        {#each choices as choice}
            <option value={choice.repoId} class="flex">
                {choice.repoId}
            </option>
        {/each}
    </select>
</div>

<style>
</style>
