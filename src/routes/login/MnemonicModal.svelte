<script lang="ts">
    import { Step, Stepper, getModalStore } from "@skeletonlabs/skeleton";
    const modalStore = getModalStore();
    $: mnemonics = $modalStore[0]?.meta?.mnemonics;

    function closeModal() {
        modalStore.close();
    }
</script>

{#if $modalStore[0]}
    <div class="card p-4 w-modal shadow-2xl space-y-6">
        {#if mnemonics.length === 1}
            <header class="card-header">
                <h3 class="h3 font-bold">Backup your seed</h3>
            </header>
            <textarea
                class="textarea font-bold mx-3"
                disabled={true}
                name="mnemonic"
                rows="2">{mnemonics[0]}</textarea
            >
            <footer class="card-footer flex justify-center">
                <button
                    class="btn variant-filled-primary font-bold"
                    on:click={closeModal}>Continue</button
                >
            </footer>
        {:else}
            <Stepper stepTerm="Share" on:complete={closeModal}>
                {#each mnemonics as mnemonic}
                    <Step>
                        <svelte:fragment slot="header"
                            >Backup your seed</svelte:fragment
                        >
                        <textarea
                            class="textarea font-bold"
                            disabled={true}
                            name="mnemonic"
                            rows="7">{mnemonic}</textarea
                        >
                    </Step>
                {/each}
            </Stepper>
        {/if}
    </div>
{/if}
