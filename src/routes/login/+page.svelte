<script lang="ts">    
    import CreateWallet from "./CreateWallet.svelte";
    import LoginWallet from "./LoginWallet.svelte";
    import { invoke } from '@tauri-apps/api';
    import { walletsNamesStore } from '../../Store';

    async function fetchNames() { 
        return await invoke("get_wallet_names")
            .then((names) => (names as string[])
                .filter((name) => !$walletsNamesStore.includes(name)))
    }
    let walletNamesPromise = fetchNames()

    let creatingNewWallet = false
</script>


{#await walletNamesPromise}
    <p>Loading wallets...</p>
{:then names}
    {#if names.length == 0 || creatingNewWallet}
        <CreateWallet loginToWallet={() => {creatingNewWallet = false}}/>
    {:else}
        <LoginWallet names={names} createNewWallet={() => { creatingNewWallet = true }} />
    {/if}
{:catch error}
    <p>{error}</p> 
{/await}
