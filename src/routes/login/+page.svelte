<script lang="ts">    
    import CreateForm from "./CreateForm.svelte";
    import LoginForm from "./LoginForm.svelte";
    import { TabGroup, Tab } from '@skeletonlabs/skeleton';
    import { invoke } from '@tauri-apps/api';
    import { walletsNamesStore } from '../../Store';
    import RecoverForm from "./RecoverForm.svelte";


    async function fetchNames() { 
        return await invoke("get_wallet_names")
            .then((names) => (names as string[])
                .filter((name) => !$walletsNamesStore.includes(name)))
    }
    let walletNamesPromise = fetchNames()
    let tabSet: number = 0;
</script>


{#await walletNamesPromise}
    <p>Loading wallets...</p>
{:then names}
    <div>
        <TabGroup justify="justify-center">
            <Tab bind:group={tabSet} name="login" value={0}>Login</Tab>
            <Tab bind:group={tabSet} name="create" value={1}>Create</Tab>
            <Tab bind:group={tabSet} name="recover" value={2}>Recover</Tab>
            <svelte:fragment slot="panel">
                {#if tabSet === 0}
                    <LoginForm names={names}/>
                {:else if tabSet === 1}
                    <CreateForm/>
                {:else if tabSet === 2}
                    <RecoverForm/>
                {/if}
            </svelte:fragment>
        </TabGroup>
    </div>
{:catch error}
    <p>{error}</p> 
{/await}
