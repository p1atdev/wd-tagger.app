<script lang="ts">
    import type { Probabilities } from "@/schema/probabilities";

    interface Props {
        probabilities: Probabilities;
    }

    let { probabilities }: Props = $props();

    const formatProb = (prob: number): string => {
        return (prob * 100).toFixed(2);
    };

    let isEmpty = Object.keys(probabilities).length === 0;
</script>

<div class="flex flex-col gap-y-3">
    {#if isEmpty}
        <p class="mx-auto text-lg my-2">Not detected</p>
    {:else}
        {#each Object.entries(probabilities) as [tag, probability]}
            <div>
                <div class="flex gap-y-0 justify-between items-center">
                    <span class="text-left text-lg font-medium">
                        {tag}
                    </span>
                    <span class="text-right text-sm"
                        >{formatProb(probability)} %</span
                    >
                </div>
                <div
                    class="mt-1 h-1 bg-blue-100 rounded-full"
                    style:width="{probability * 100}%"
                ></div>
            </div>
        {/each}
    {/if}
</div>
