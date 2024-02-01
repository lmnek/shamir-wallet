<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import { goto } from "$app/navigation";
    import {
        getModalStore,
        SlideToggle,
        type ModalSettings,
    } from "@skeletonlabs/skeleton";
    import { RangeSlider } from "@skeletonlabs/skeleton";
    import showErrorToast from "$lib/ErrorToast";
    const modalStore = getModalStore();
    import { getToastStore, type ToastStore } from "@skeletonlabs/skeleton";
    import LoadingButton from "$lib/LoadingButton.svelte";
    const toastStore: ToastStore = getToastStore();

    let name = "";
    let password = "";
    let shamirOn = false;
    const maxShares = 16;
    let numberOfShares = 2;
    let sharesTreshold = 1;
    $: maxTreshold = numberOfShares;

    let handleSubmit = async () => {
        if (name == "" || password == "") {
            showErrorToast("Enter nonempty name and password", toastStore);
            return;
        }
        if (!shamirOn) {
            await invoke<string>("cw", { name, password })
                .then((mnemonic) => {
                    const modal: ModalSettings = {
                        type: "component",
                        component: "mnemonicModal",
                        meta: { mnemonics: [mnemonic] },
                    };
                    modalStore.trigger(modal);
                    goto(`/wallet/${name}`);
                })
                .catch((err) => showErrorToast(err, toastStore));
        } else {
            await invoke<string[]>("cw_shamir", {
                name,
                password,
                treshold: sharesTreshold,
                count: numberOfShares,
            })
                .then((mnemonics) => {
                    const modal: ModalSettings = {
                        type: "component",
                        component: "mnemonicModal",
                        meta: { mnemonics: mnemonics },
                    };
                    modalStore.trigger(modal);
                    goto(`/wallet/${name}`);
                })
                .catch((err) => showErrorToast(err, toastStore));
        }
    };
</script>

<div class="flex flex-col mb-5 items-center">
    <h3 class="m-3 h3">Create new wallet</h3>
    <form class="p-4 max-w-md mx-auto space-y-5">
        <div>
            <label for="name" class="label block text-sm font-bold mb-2"
                >Name</label
            >
            <input
                type="text"
                id="name"
                bind:value={name}
                class="input"
                placeholder="Your name"
            />
        </div>

        <div>
            <label for="password" class="label block text-sm font-bold mb-2"
                >Password</label
            >
            <input
                type="password"
                id="password"
                bind:value={password}
                class="input"
                placeholder="******************"
            />
        </div>

        <SlideToggle
            name="slide-label"
            active="bg-primary-500"
            bind:checked={shamirOn}>Shamir backup</SlideToggle
        >
        {#if shamirOn}
            <RangeSlider
                name="range-slider"
                bind:value={numberOfShares}
                max={maxShares}
                min={2}
                step={1}
                ticked
            >
                <div class="flex justify-between items-center">
                    <div class="font-bold">Number of shares</div>
                    <div class="text-xs">{numberOfShares} / {maxShares}</div>
                </div>
            </RangeSlider>
            <RangeSlider
                name="range-slider"
                bind:value={sharesTreshold}
                max={maxTreshold}
                min={1}
                step={1}
                ticked
            >
                <div class="flex justify-between items-center">
                    <div class="font-bold">Treshold for recovery</div>
                    <div class="text-xs">{sharesTreshold} / {maxTreshold}</div>
                </div>
            </RangeSlider>
        {/if}
        <LoadingButton text="Create new wallet" onClick={handleSubmit} />
    </form>
</div>
