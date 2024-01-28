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
    let numberOfShares = 2
    let sharesTreshold = 1
    $: maxTreshold = numberOfShares

    let handleSubmit = async () => {
        if (name == "" || password == "") {
            error = "Enter nonempty name and password"
            return
        }
        // TODO: better display of mnemonics in popup
        if(!shamirOn){
            await invoke<string>("cw", { name, password })
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
        } else {
            await invoke<string[]>("cw_shamir", { name, password, treshold: sharesTreshold, count: numberOfShares  })
            .then((mnemonics) => {
                const modal: ModalSettings = {
                    type: 'alert',
                    title: 'Mnemonic phrase',
                    body: 'SAVE THE MNEMONIC PHRASE: ' + mnemonics.map(mn => "SHARE: " + mn + "\n"),
                    buttonTextCancel: 'OK'
                };
                modalStore.trigger(modal);
                goto(`/wallet/${name}`)
            })
            .catch((err) => error = err)
        }
    }
</script>

<div class="flex flex-col mb-32 items-center">
    <h1 class="text-xl m-3">Create a new wallet</h1>
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
        {#if shamirOn}
            <RangeSlider name="range-slider" bind:value={numberOfShares} max={maxShares} min={2} step={1} ticked>
                <div class="flex justify-between items-center">
                    <div class="font-bold">Number of shares</div>
                    <div class="text-xs">{numberOfShares} / {maxShares}</div>
                </div>
            </RangeSlider>
            <RangeSlider name="range-slider" bind:value={sharesTreshold} max={maxTreshold} min={1} step={1} ticked>
                <div class="flex justify-between items-center">
                    <div class="font-bold">Treshold for recovery</div>
                    <div class="text-xs">{sharesTreshold} / {maxTreshold}</div>
                </div>
            </RangeSlider>
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
