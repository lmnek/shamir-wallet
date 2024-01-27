<script lang="ts">
    import { goto } from "$app/navigation";
    import { walletsNamesStore, walletsCacheStore } from "../../../Store";
    import type { WalletData } from "../../../Types";
    import { invoke } from "@tauri-apps/api";
    import { get } from "svelte/store";
    export let data: { name: string } 

    let walletData: WalletData | undefined 
    $: if(data) {
        loadWalletData(data.name)
    }

    async function loadWalletData(name: string) {
        // without subscribing
        const walletNames = get(walletsNamesStore);
        const cachedWallets = get(walletsCacheStore);
        // in cache?
        const cachedData = cachedWallets.find(wallet => wallet.name === name);
        if (walletNames.includes(name) && cachedData) {
            walletData = cachedData;
            return;
        }
        console.log(name)
        await invoke<WalletData>('get_wallet_data', { 'name': name })
            .then((dt) => {
                walletData = dt;
                walletsNamesStore.update(names => [name, ...names]);
                walletsCacheStore.update(wallets => [dt, ...wallets]);
            }); // TODO: catch
    }

    async function onSynchronize() {
        loadWalletData(data.name)
    }

    async function onDelete() {
        let name = data.name
        await invoke('delete_wallet', { name })
        walletsNamesStore.update(ws => ws.filter(w => w != name))
        walletsCacheStore.update(ws => ws.filter(w => w.name != name))
        goto('/')
    }

    async function onClose() {
        let name = data.name
        await invoke('close_wallet', { name })
        walletsNamesStore.update(ws => ws.filter(w => w != name))
        walletsCacheStore.update(ws => ws.filter(w => w.name != name))
        goto('/')
    }

    // TODO: send popup
    // TODO: transactions display
</script>

{#if walletData == undefined}
    <div>Loading...</div>
{:else}
    <div>{walletData.name}</div>
    <div class="text-7xl text-bold text-center">{walletData.balance} BTC</div>
    <div>Unused address: {walletData.address}</div> 
    {#each walletData.transactions as transaction}
        <p>From {transaction.sent} received {transaction.received}</p>
    {/each}
    <button>Send</button>
    <button on:click={onClose}>Close</button>
    <button on:click={onDelete}>Delete wallet</button>
    <button on:click={onSynchronize}>Synchronize</button>
{/if}
