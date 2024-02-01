<script lang="ts">
    import { tick } from "svelte";
    import { ProgressRadial } from "@skeletonlabs/skeleton";

    export let text: string;
    export let onClick: () => Promise<void> = async () => {};
    export let color: string = "variant-filled-primary";
    export let meter: string = "stroke-surface-900";

    let promise: Promise<void> | null = null;

    async function onClickWithLoading() {
        promise = onClick();
        await tick();
        await promise;
        promise = null;
    }
</script>

<div class="flex justify-center items-center">
    <button class="btn {color} h-12 font-bold" on:click={onClickWithLoading}>
        {#if promise}
            <div class="flex justify-center items-center p-3">
                <ProgressRadial value={undefined} strokeLinecap="round" stroke={120} width="w-8" meter={meter}/>
            </div>
        {:else}
            {text}
        {/if}
    </button>
</div>
