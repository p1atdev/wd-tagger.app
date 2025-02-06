<script lang="ts">
    import Icon from "@iconify/svelte";

    interface Props {
        choices: string[];
        selected: string;
    }

    let { choices, selected = $bindable() }: Props = $props();
</script>

<div class="p-2">
    <label
        for="model-select"
        class="block text-sm mb-1 text-gray-900 dark:text-white">Model</label
    >
    <select
        bind:value={selected}
        id="model-select"
        class="text-lg w-full pl-2 pr-8 py-2 rounded-sm border border-blue-300 dark:border-blue-600 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
    >
        {#each choices as choice}
            <option value={choice} class="flex">
                <Icon icon="mdi:chevron-down" class="mr-2" />
                {choice}
            </option>
        {/each}
    </select>
</div>

<style>
</style>
