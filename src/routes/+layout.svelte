<script lang="ts">
    import "../app.postcss";
    import {
        AppShell,
        AppBar,
        Modal,
        AppRail,
        AppRailAnchor,
    } from "@skeletonlabs/skeleton";
    import { initializeStores } from "@skeletonlabs/skeleton";
    import { walletsNamesStore } from "../Store";
    import { page } from "$app/stores";
    initializeStores(); // for modal

    $: wallets = $walletsNamesStore;
</script>

<Modal />

<AppShell>
    <svelte:fragment slot="header">
        <AppBar>
            <svelte:fragment slot="lead">
                <strong class="text-xl uppercase">Shamir Wallet</strong>
            </svelte:fragment>
            <svelte:fragment slot="trail">
                <a
                    class="btn btn-sm variant-ghost-surface"
                    href="https://github.com/skeletonlabs/skeleton"
                    target="_blank"
                    rel="noreferrer"
                >
                    GitHub
                </a>
            </svelte:fragment>
        </AppBar>
    </svelte:fragment>

    <svelte:fragment slot="sidebarLeft">
        <AppRail>
            <svelte:fragment slot="lead">
                <AppRailAnchor href="/" selected={$page.url.pathname === "/"}
                    >Home</AppRailAnchor
                >
            </svelte:fragment>
            {#each wallets as wallet (wallet)}
                <AppRailAnchor
                    class="border-white p-2"
                    href="/wallet/{wallet}"
                    selected={$page.url.pathname.endsWith(wallet)}
                    >{wallet}</AppRailAnchor
                >
            {/each}
            <svelte:fragment slot="trail">
                <AppRailAnchor href="/" target="_blank" title="Settings"
                    >Settings</AppRailAnchor
                >
            </svelte:fragment>
        </AppRail>
    </svelte:fragment>

    <div
        class="container h-full mx-auto flex flex-col items-center m-5"
    >
        <slot />
    </div>
</AppShell>
