<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { goto } from '$app/navigation';
    import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
    const modalStore = getModalStore();

    export let loginToWallet: () => void
    let name = ""
    let password = ""
    let error = ""

    let handleSubmit = async () => {
        if (name == "") {
            error = "Enter nonempty name"
            return
        }
        await invoke("cw", { name, password })
        .then((mnemonic) => {
            const modal: ModalSettings = {
                type: 'alert',
                title: 'Mnemonic phrase',
                body: 'SAVE THE MNEMONIC PHRASE: ' + mnemonic,
                buttonTextCancel: 'OK'
            };
            modalStore.trigger(modal);
            goto(`/wallet/${name}`)
        })
        .catch((err) => error = err)
    }
</script>

<div class="flex flex-col mb-32 items-center">
    <h1>Create new wallet</h1>
    <form on:submit|preventDefault={handleSubmit} class="p-4 max-w-md mx-auto space-y-5">
        <div>
            <label for="name" class="label block text-sm font-bold mb-2">Name</label>
            <input 
                type="text" 
                id="name" 
                bind:value={name} 
                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" 
                placeholder="Your name"
            />
        </div>

        <div>
            <label for="password" class="label block text-sm font-bold mb-2">Password</label>
            <input 
                type="password" 
                id="password" 
                bind:value={password} 
                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline" 
                placeholder="******************"
            />
        </div>

        <div class="flex flex-col justify-center">
            {#if error != ""}
                <p>{error}</p>
            {/if}
            <button 
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" 
                type="submit"
            >Create new wallet</button>
            <button 
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" 
            >TODO: load from seed</button>
        </div>
    </form>
        <button on:click={loginToWallet}>Login to wallet</button>
</div>
