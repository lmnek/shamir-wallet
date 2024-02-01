<script lang="ts">
    import CreateForm from "./CreateForm.svelte";
    import LoginForm from "./LoginForm.svelte";
    import { TabGroup, Tab } from "@skeletonlabs/skeleton";
    import { invoke } from "@tauri-apps/api";
    import { walletsNamesStore } from "../../Store";
    import RecoverForm from "./RecoverForm.svelte";
    import showErrorToast from "$lib/ErrorToast";
    import { getToastStore, type ToastStore } from "@skeletonlabs/skeleton";
    import { onMount } from "svelte";
    const toastStore: ToastStore = getToastStore();

    let names: string[] = [];
    let tabSet: number = 0;

    onMount(async () => {
        // fetch wallet names
        names = await invoke("get_wallet_names")
            .then((names) =>
                (names as string[]).filter(
                    (name) => !$walletsNamesStore.includes(name),
                ),
            )
            .catch((err) => {
                showErrorToast(err, toastStore);
                return [];
            });
    })
</script>

<div class="container mx-auto h-full flex flex-col items-center">
    <TabGroup justify="justify-center mt-3">
        <Tab bind:group={tabSet} class="font-bold" name="login" value={0}>Login</Tab>
        <Tab bind:group={tabSet} class="font-bold"name="create" value={1}>Create</Tab>
        <Tab bind:group={tabSet} class="font-bold"name="recover" value={2}>Recover</Tab>
        <svelte:fragment slot="panel">
            {#if tabSet === 0}
                <LoginForm {names} />
                {:else if tabSet === 1}
                <CreateForm />
                {:else if tabSet === 2}
                <RecoverForm />
            {/if}
        </svelte:fragment>
    </TabGroup>
</div>
