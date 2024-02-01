<script lang="ts">
    import "../app.postcss";
    import {
        AppShell,
        AppBar,
        Modal,
        type ModalComponent,
        Toast,
    } from "@skeletonlabs/skeleton";
    import { initializeStores } from "@skeletonlabs/skeleton";
    import { Drawer, getDrawerStore } from "@skeletonlabs/skeleton";
    import SendModal from "./wallet/[name]/send/SendModal.svelte";
    import BitcoinSvg from "$lib/BitcoinSvg.svelte";
    import Fa from "svelte-fa";
    import { faBars } from "@fortawesome/free-solid-svg-icons";
    import SideBar from "./SideBar.svelte";
    import MnemonicModal from "./login/MnemonicModal.svelte";

    // For modals
    initializeStores();
    const modalRegistry: Record<string, ModalComponent> = {
        sendModal: { ref: SendModal },
        mnemonicModal: { ref: MnemonicModal }
    };
    const drawerStore = getDrawerStore();

    function drawerOpen() {
        drawerStore.open()
    }
</script>

<Modal components={modalRegistry} />
<Toast position="br" />
<Drawer width="w-1/3">
    <SideBar/>
</Drawer>

<AppShell slotSidebarLeft="w-0 lg:w-72 bg-surface-500/10">
    <svelte:fragment slot="header">
        <AppBar
            gridColumns="grid-cols-3"
            slotDefault="place-self-center"
            slotTrail="place-content-end"
        >
            <svelte:fragment slot="lead">
                <div class="flex items-center">
                    <button class="btn lg:hidden" on:click={drawerOpen}>
                        <Fa icon={faBars} size="2x" />
                    </button>
                    <strong class="ml-3 text-2xl uppercase whitespace-nowrap"
                        >Shamir Wallet</strong
                    >
                </div>
            </svelte:fragment>
            <svelte:fragment slot="trail">
                <div class="self-center justify-self-center">
                    <BitcoinSvg size={50} />
                </div>
            </svelte:fragment>
        </AppBar>
    </svelte:fragment>

    <svelte:fragment slot="sidebarLeft">
        <SideBar/>
    </svelte:fragment>
    <slot />
</AppShell>
