<script lang="ts">
    import "../app.postcss";
    import {
        AppShell,
        AppBar,
        Modal,
        AppRail,
        AppRailAnchor,
        type ModalComponent,
        Toast,
    } from "@skeletonlabs/skeleton";
    import { initializeStores } from "@skeletonlabs/skeleton";
    import { walletsNamesStore } from "../Store";
    import { page } from "$app/stores";
    import SendModal from "./wallet/[name]/send/SendModal.svelte";
    import BitcoinSvg from "$lib/BitcoinSvg.svelte";
    import Fa from 'svelte-fa'
    import { faBars } from '@fortawesome/free-solid-svg-icons'

    // For modals
    initializeStores();
    const modalRegistry: Record<string, ModalComponent> = {
        sendModal: { ref: SendModal },
    };

    $: wallets = $walletsNamesStore;
</script>

<Modal components={modalRegistry} />
<Toast position="br" />

<AppShell>
    <svelte:fragment slot="header">
        <AppBar
            gridColumns="grid-cols-3"
            slotDefault="place-self-center"
            slotTrail="place-content-end"
        >
            <svelte:fragment slot="lead">
                <div class="flex items-center">
                    <Fa icon={faBars} size="2x"/>
                    <strong class="ml-5 text-2xl uppercase">Shamir Wallet</strong>
                </div>
            </svelte:fragment>
            <svelte:fragment slot="trail">
                <div class="self-center justify-self-center">
                    <BitcoinSvg size={50}/>
                </div>
            </svelte:fragment>
        </AppBar>
    </svelte:fragment>

    <svelte:fragment slot="sidebarLeft">
        <AppRail background="bg-surface-500">
            <svelte:fragment slot="lead">
                <AppRailAnchor
                    href="/login"
                    selected={$page.url.pathname === "/login"}
                    >Open wallet</AppRailAnchor
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
    <slot />
</AppShell>
