<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { page } from "$app/stores";
    import ButtonBack from "../ButtonBack.svelte";
    import { RangeSlider, type ModalSettings } from "@skeletonlabs/skeleton";
    import { walletsCacheStore } from "../../../../Store";
    import { getModalStore } from "@skeletonlabs/skeleton";
    import showErrorToast from "$lib/ErrorToast";
    const modalStore = getModalStore();
    import { getToastStore, type ToastStore } from "@skeletonlabs/skeleton";
    const toastStore: ToastStore = getToastStore()

    // HACK: used regex to parse URL, is there a cleaner way in Svelte?
    let walletName = $page.url.pathname.match("/wallet/(.*?)/")![1];
    let walletData = $walletsCacheStore.find(
        (wallet) => wallet.name === walletName,
    )!;

    let amount: number | undefined = undefined;
    let address = "";
    let feeRate = 1.0;

    async function handleSubmit() {
        if(amount == undefined || amount <= 0) {
            showErrorToast("Amount has to be a positive number", toastStore)
            return
        }
        if(address == "") {
            showErrorToast("Address is empty", toastStore)
            return
        }

        // TODO: delete afterwards
        const tx_details = { amount, address, feeRate, name: walletName };
        const modal: ModalSettings = {
            type: "component",
            component: "sendModal",
            meta: { tx_details, fee: 30 }
        };
        modalStore.trigger(modal);

        //const tx_details = { amount, address, feeRate, name: walletName };
        //await invoke<number>("fee_for_transaction", tx_details)
        //    .then((fee) => {
        //        const modal: ModalSettings = {
        //            type: "component",
        //            component: "sendModal",
        //            meta: { tx_details, fee },
        //        };
        //        modalStore.trigger(modal);
        //    })
        //    .catch((err) => showErrorToast(err, toastStore));
    }
</script>

<ButtonBack url={$page.url.pathname} />
<h1 class="h3 mb-3">Send a transaction</h1>
<form
    on:submit|preventDefault={handleSubmit}
    class="p-4 max-w-md mx-auto space-y-10"
>
    <div>
        <div class="flex justify-between items-center">
            <label for="amount" class="label block text-sm font-bold mb-2"
                >Amount</label
            >
            <label for="amount" class="label block text-sm font-bold mb-2">
                Balance: {walletData.balance}</label
            >
        </div>
        <input
            type="number"
            id="amount"
            bind:value={amount}
            class="input"
            placeholder="Enter amount (sats)"
        />
    </div>

    <div>
        <label for="address" class="label block text-sm font-bold mb-2"
            >Adress</label
        >
        <input
            type="text"
            id="address"
            bind:value={address}
            class="input"
            placeholder="Enter a bitcoin address"
        />
    </div>

    <RangeSlider
        name="range-slider"
        bind:value={feeRate}
        max={100}
        min={1}
        step={0.1}
        ticked
    >
        <div class="flex justify-between items-center">
            <div class="font-bold">Fees (sats/vbyte)</div>
            <div class="text-xs">{feeRate}</div>
        </div>
    </RangeSlider>

    <div class="flex flex-col justify-center">
        <button
            class="btn variant-filled-secondary font-bold"
            type="submit">Verify transaction</button
        >
    </div>
</form>
