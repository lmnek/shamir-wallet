<script lang="ts">
    import { goto } from "$app/navigation";
    import {
        getModalStore,
        getToastStore,
        type ToastSettings,
        type ToastStore,
    } from "@skeletonlabs/skeleton";
    import { invoke } from "@tauri-apps/api/tauri";
    import showErrorToast from "$lib/ErrorToast";
    const modalStore = getModalStore();
    const toastStore: ToastStore = getToastStore();

    $: tx_details = $modalStore[0]?.meta?.tx_details;

    async function onConfirm() {
        await invoke("send", tx_details)
            .then(() => {
                const settings: ToastSettings = {
                    message: "Transaction succesfully transmitted",
                    background: "variant-filled-success",
                };
                toastStore.trigger(settings);
                goto("/wallet/" + tx_details.name);
                modalStore.close();
            })
            .catch((err) => showErrorToast(err, toastStore));
    }
</script>

{#if $modalStore[0]}
    <div class="card p-4 w-modal shadow-2xl space-y-6">
        <header class="card-header">
            <h3 class="h3 font-bold text-primary-500">Transaction verification</h3>
        </header>
        <section class="p-4 space-y-5">
            <div class="flex items-center justify-between">
                <h4 class="h4">Amount:</h4>
                <p class="text-right font-bold">{tx_details.amount} sats</p>
            </div>
            <div class="flex items-center justify-between">
                <h4 class="h4">Address:</h4>
                <p class="text-right font-bold">{tx_details.address}</p>
            </div>
            <div class="flex items-center justify-between">
                <h4 class="h4">Fee:</h4>
                <p class="text-right font-bold">{$modalStore[0].meta.fee}</p>
            </div>
        </section>
        <footer class="card-footer flex justify-center">
            <button class="btn variant-filled-primary font-bold" on:click={onConfirm}
                >Confirm transaction</button
            >
        </footer>
    </div>
{/if}
