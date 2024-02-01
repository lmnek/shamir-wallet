<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { walletsNamesStore, walletsCacheStore } from "../../../Store";
    import type { WalletData } from "../../../Types";
    import { invoke } from "@tauri-apps/api";
    import { get } from "svelte/store";
    import showErrorToast from "$lib/ErrorToast";
    import {
        getToastStore,
        ProgressRadial,
        type ModalSettings,
        type ToastStore,
        getModalStore,
        type ToastSettings,
    } from "@skeletonlabs/skeleton";
    import LoadingButton from "$lib/LoadingButton.svelte";
    import Fa from "svelte-fa";
    import {
    faArrowUpFromBracket,
        faRotate,
        faTable,
        faTableList,
        faTrash,
        faXmark,
    } from "@fortawesome/free-solid-svg-icons";
    const toastStore: ToastStore = getToastStore();
    const modalStore = getModalStore();

    export let data: { name: string };
    $: walletName = data.name;
    $: walletDataPromise = initialFetch(walletName);

    async function initialFetch(walletName: string) {
        // without subscribing
        const walletNames = get(walletsNamesStore);
        const cachedWallets = get(walletsCacheStore);
        // in cache?
        const cachedData = cachedWallets.find(
            (wallet) => wallet.name === walletName,
        );
        if (walletNames.includes(walletName) && cachedData) {
            return cachedData;
        }
        let dt = await invoke<WalletData>("get_wallet_data", {
            name: walletName,
        });
        walletsNamesStore.update((names) => [walletName, ...names]);
        walletsCacheStore.update((wallets) => [dt, ...wallets]);
        return dt;
    }

    async function onSynchronize() {
        let dt = await invoke<WalletData>("get_wallet_data", {
            name: walletName,
        });
        if (!get(walletsNamesStore).includes(walletName)) {
            walletsNamesStore.update((names) => [walletName, ...names]);
            walletsCacheStore.update((wallets) => [dt, ...wallets]);
        } else {
            walletsCacheStore.update((wallets) =>
                wallets.map((w) => (w.name === walletName ? dt : w)),
            );
        }
        const settings: ToastSettings = {
            message: "Blockchain synchronization completed",
            background: "variant-filled-success",
        };
        toastStore.trigger(settings);
        let getDt = async () => dt;
        walletDataPromise = getDt();
    }

    async function onDelete() {
        let name = walletName;

        const modal: ModalSettings = {
            type: "confirm",
            title: "Confirm deletion",
            body: "Are you sure you wish to delete this wallet?",
            buttonTextConfirm: "Delete",
            response: async (confirmed: boolean) => {
                if (confirmed) {
                    await invoke("delete_wallet", { name }).catch((err) =>
                        showErrorToast(err, toastStore),
                    );
                    walletsNamesStore.update((ws) =>
                        ws.filter((w) => w != name),
                    );
                    walletsCacheStore.update((ws) =>
                        ws.filter((w) => w.name != name),
                    );
                    const settings: ToastSettings = {
                        message: "Wallet succesfully deleted",
                        background: "variant-filled-success",
                    };
                    toastStore.trigger(settings);
                    goto("/login");
                }
            },
        };
        modalStore.trigger(modal);
    }

    async function onClose() {
        let name = walletName;
        await invoke("close_wallet", { name }).catch((err) =>
            showErrorToast(err, toastStore),
        );
        walletsNamesStore.update((ws) => ws.filter((w) => w != name));
        walletsCacheStore.update((ws) => ws.filter((w) => w.name != name));
        goto("/login");
    }
</script>

{#await walletDataPromise}
    <div class="flex flex-col h-full justify-center items-center">
        <div class="transform -translate-y-20">
            <ProgressRadial
                value={undefined}
                strokeLinecap="round"
                stroke={120}
                width="w-44"
                meter="stroke-primary-500"
            />
            <h3 class="h3 mt-10 text-center">Loading the wallet</h3>
        </div>
    </div>
{:then walletData}
    <div class="flex flex-grow justify-between items-center w-full mx-3 mt-4">
        <div class="flex w-1/3 space-x-3">
            <button class="btn variant-ghost-secondary" on:click={onClose}>
                <Fa icon={faXmark} size="1.2x" />
                <p>Close</p>
            </button>
            <button class="btn variant-soft-secondary" on:click={onDelete}>
                <Fa icon={faTrash} size="1x" />
                <p>Delete</p>
            </button>
        </div>
        <h1 class="text-center h3 w-1/3 font-bold text-primary-500">
            {walletData.name}
        </h1>
        <div class="flex w-1/3 justify-end">
            <LoadingButton
                text="Synchronize"
                onClick={onSynchronize}
                color="variant-filled-secondary"
                meter="stroke-primary-50"
                icon={faRotate}
            />
        </div>
    </div>
    <div class="flex h-full flex-col items-center space-y-9">
        <div class="text-6xl my-3 text-bold text-center">
            {walletData.balance} sats
        </div>
        <div class="flex flex-col justify-center items-center w-full">
            <code class="code">{walletData.address}</code>
            <p>(last unused address)</p>
        </div>
        <button
            class="btn variant-filled-primary font-bold"
            on:click={() => goto($page.url.pathname + "/send")}
        >
            <Fa icon={faArrowUpFromBracket} size="1.3x" />
            <p>Send</p>
        </button>
        <button
            class="btn variant-filled-primary font-bold"
            on:click={() => goto($page.url.pathname + "/transactions")}
        >
            <Fa icon={faTableList} size="1.3x" />
            <p>Transactions</p>
        </button>
    </div>
{:catch error}
    <h2>Error: {error}</h2>
{/await}
