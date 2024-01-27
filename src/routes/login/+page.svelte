<script lang="ts">    
    import CreateWallet from "./CreateWallet.svelte";
    import LoginWallet from "./LoginWallet.svelte";
    import { invoke } from '@tauri-apps/api';

    async function fetchNames() { 
        return await invoke("get_wallet_names")
            .then((names) => names as string[])
    }
    let walletNamesPromise = fetchNames()

    let creatingNewWallet = false
</script>


<div class="container h-full mx-auto flex justify-center items-center">
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
</div>
