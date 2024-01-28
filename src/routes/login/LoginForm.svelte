<script lang="ts">
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api";

    export let names: Array<String>

    let selectedWalletName = ""
    let password = ""
    let error = ""

    let handleSubmit = async () => {
        await invoke("load_wallet", { name: selectedWalletName, password })
        .then(() => goto(`/wallet/${selectedWalletName}`))
        .catch((err) => error = err)
    }
</script>


<div class="flex flex-col mb-32 items-center">
    <h1 class="text-xl m-3">Login to existing wallet</h1>
    <form on:submit|preventDefault={handleSubmit} class="p-4 max-w-md mx-auto space-y-5">
        <div class="mb-4">
            <label for="selectName" class="block text-sm font-bold mb-2">Select a Name</label>
            <select
                id="selectName"
                bind:value={selectedWalletName}
                class="shadow border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            >
                <option value="" disabled>Select wallet</option>
                {#each names as name}
                    <option value={name}>{name}</option>
                {/each}
            </select>
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
            >Login to wallet</button>
        </div>
    </form>
</div>
