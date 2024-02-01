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
    import { getToastStore, type ToastStore } from "@skeletonlabs/skeleton";
    import LoadingButton from "$lib/LoadingButton.svelte";
    const toastStore: ToastStore = getToastStore();

    let name = "";
    let password = "";
    let shamirOn = false;
    const maxShares = 16;
    let sharesCount = 1;
    let mnemonic = "";
    let mnemonics: string[] = Array(sharesCount).fill("");
    $: if (shamirOn) {
        mnemonics = Array.from(
            { length: sharesCount },
            (_, i) => mnemonics[i] || "",
        );
    }

    let handleSubmit = async () => {
        if (
            name == "" ||
            password == "" ||
            (!shamirOn && mnemonic == "") ||
            (shamirOn && mnemonics.includes(""))
        ) {
            showErrorToast("Detected empty value", toastStore);
            return;
        }
        if (!shamirOn) {
            await invoke<string>("rw", { name, password, mnemonic })
                .then((_) => {
                    goto(`/wallet/${name}`);
                })
                .catch((err) => showErrorToast(err, toastStore));
        } else {
            let mnemnonics_split = mnemonics.map((mn) => mn.split(" "));
            await invoke<string[]>("rw_shamir", {
                name,
                password,
                mnemonics: mnemnonics_split,
            })
                .then((_) => {
                    goto(`/wallet/${name}`);
                })
                .catch((err) => showErrorToast(err, toastStore));
        }
    };
</script>

<div class="flex flex-col mb-32 items-center">
    <h3 class="m-3 h3">Recover wallet</h3>
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
        {#if !shamirOn}
            <div>
                <label for="mnemonic" class="label block text-sm font-bold mb-2"
                    >Mnemonic phrase</label
                >
                <textarea
                    id="mnemonic"
                    bind:value={mnemonic}
                    class="textarea"
                    rows="3"
                    placeholder="Enter the words"
                />
            </div>
        {:else}
            <RangeSlider
                name="range-slider"
                bind:value={sharesCount}
                max={maxShares}
                min={1}
                step={1}
                ticked
            >
                <div class="flex justify-between items-center">
                    <div class="font-bold">Number of shares</div>
                    <div class="text-xs">{sharesCount} / {maxShares}</div>
                </div>
            </RangeSlider>

            <label for="mnemonics" class="label block text-sm font-bold mb-2"
                >Mnemonic phrases</label
            >
            {#each mnemonics as _, i}
                <div>
                    <textarea
                        id="mnemonic"
                        bind:value={mnemonics[i]}
                        class="textarea"
                        rows="5"
                        placeholder="Share #{i + 1}"
                    />
                </div>
            {/each}
        {/if}
        <LoadingButton text="Create new wallet" onClick={handleSubmit} />
    </form>
</div>
