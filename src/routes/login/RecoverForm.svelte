<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { goto } from '$app/navigation';
    import { getModalStore, SlideToggle, type ModalSettings } from '@skeletonlabs/skeleton';
    import { RangeSlider } from '@skeletonlabs/skeleton';
    const modalStore = getModalStore();

    let name = ""
    let password = ""
    let error = ""
    let shamirOn = false
    const maxShares = 16
    let sharesCount = 1
    let mnemonic = ""
    let mnemonics: string[] = Array(sharesCount).fill('') 
    $: if (shamirOn) {
        mnemonics = Array.from({ length: sharesCount }, (_, i) => mnemonics[i] || '')
    }

    let handleSubmit = async () => {
        if (name == "" || password == "") {
            error = "Enter nonempty name and password"
            return
        }
        if(!shamirOn){
            await invoke<string>("rw", { name, password, mnemonic })
            .then((_) => { goto(`/wallet/${name}`) })
            .catch((err) => error = err)
        } else {
            let mnemnonics_split = mnemonics.map(mn => mn.split(" "))
            await invoke<string[]>("rw_shamir", { name, password, mnemonics: mnemnonics_split })
            .then((_) => { goto(`/wallet/${name}`) })
            .catch((err) => error = err)
        }
    }
</script>

<div class="flex flex-col mb-32 items-center">
    <h1 class="text-xl m-3">Recover a wallet</h1>
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

        <SlideToggle name="slide-label" bind:checked={shamirOn}>Shamir backup</SlideToggle>
        {#if !shamirOn}
            <div>
                <label for="mnemonic" class="label block text-sm font-bold mb-2">Mnemonic phrase</label>
                <textarea
                    id="mnemonic" 
                    bind:value={mnemonic} 
                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline" 
                    placeholder="Enter the words"
                />
            </div>
        {:else}
            <RangeSlider name="range-slider" bind:value={sharesCount} max={maxShares} min={1} step={1} ticked>
                <div class="flex justify-between items-center">
                    <div class="font-bold">Number of shares</div>
                    <div class="text-xs">{sharesCount} / {maxShares}</div>
                </div>
            </RangeSlider>

            <label for="mnemonics" class="label block text-sm font-bold mb-2">Mnemonic phrases</label>
            {#each mnemonics as _, i}
                <div>
                    <textarea
                        id="mnemonic" 
                        bind:value={mnemonics[i]} 
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline" 
                        placeholder="Share #{i + 1}"
                    />
                </div>
            {/each}
        {/if}

        <div class="flex flex-col justify-center">
            {#if error != ""}
                <p>{error}</p>
            {/if}
            <button 
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" 
                type="submit"
            >Create new wallet</button>
        </div>
    </form>
</div>
