<script lang="ts">
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api";
    import showErrorToast from "$lib/ErrorToast";
    import { getToastStore, type ToastStore } from "@skeletonlabs/skeleton";
    import LoadingButton from "$lib/LoadingButton.svelte";
    const toastStore: ToastStore = getToastStore();

    export let names: Array<String>;

    let selectedWalletName = "";
    let password = "";

    let handleSubmit = async () => {
        if (password == "") {
            showErrorToast("Password cannot be empty", toastStore);
            return;
        }
        await invoke("load_wallet", { name: selectedWalletName, password })
            .then(() => goto(`/wallet/${selectedWalletName}`))
            .catch((err) => {
                showErrorToast(err, toastStore);
            });
    };
</script>

<div class="flex flex-col items-center">
    <h3 class="m-3 h3">Login to existing wallet</h3>
    <form class="p-4 max-w-md mx-auto space-y-5">
        <div class="mb-4">
            <label for="selectName" class="block text-sm font-bold mb-2"
                >Name</label
            >
            <select
                id="selectName"
                bind:value={selectedWalletName}
                class="select"
            >
                <option value="" disabled>Select wallet</option>
                {#each names as name}
                    <option value={name}>{name}</option>
                {/each}
            </select>
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
        <LoadingButton text="Login to wallet" onClick={handleSubmit} />
    </form>
</div>

<style>
    .option-custom {
        background-color: red;
    }
</style>
