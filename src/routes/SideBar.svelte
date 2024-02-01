<script lang="ts">
    import { page } from "$app/stores";
    import Fa from "svelte-fa";
    import { walletsNamesStore } from "../Store";
    import {
        faAngleRight,
        faGear,
        faHome,
    } from "@fortawesome/free-solid-svg-icons";
    import { faGithub } from "@fortawesome/free-brands-svg-icons";
    import { open } from "@tauri-apps/api/shell";
    import { getDrawerStore } from "@skeletonlabs/skeleton";
    const drawerStore = getDrawerStore();

    $: getIconColor = (walletName: string) =>
        $page.url.pathname.startsWith("/wallet/" + walletName)
            ? "orange"
            : "white";

    function drawerClose() {
        drawerStore.close();
    }
</script>

<nav class="list-nav justify-between h-full flex flex-col p-3">
    <div>
        <h3 class="h3 font-bold m-5 self-center lg:hidden">Navigation</h3>
        <ul>
            <li>
                <a href="/login" on:click={drawerClose}>
                    <span class="badge bg-secondary-500 aspect-square">
                        <Fa icon={faHome} size="1.6x" />
                    </span>
                    <span class="flex-auto text-xl">Login to wallet</span>
                </a>
            </li>
            <hr class="!border-t-4" />
            {#each $walletsNamesStore as wallet (wallet)}
                <li>
                    <a href="/wallet/{wallet}" on:click={drawerClose}>
                        <span class="badge aspect-square">
                            <Fa
                                icon={faAngleRight}
                                size="2.5x"
                                color={getIconColor(wallet)}
                            />
                        </span>
                        <span class="flex-auto text-xl">{wallet}</span>
                    </a>
                </li>
            {/each}
        </ul>
    </div>
    <div>
        <hr class="!border-t-4" />
        <a href="/login" on:click={drawerClose}>
            <span class="badge bg-secondary-500 aspect-square">
                <Fa icon={faGear} size="2x" />
            </span>
            <span class="flex-auto text-xl">Settings</span>
        </a>
        <a
            href="/"
            on:click|preventDefault={async () =>
                await open("https://github.com/lmnek/shamir-wallet")}
        >
            <span class="badge bg-secondary-500 aspect-square">
                <Fa icon={faGithub} size="2x" />
            </span>
            <span class="flex-auto text-xl">Github</span>
        </a>
    </div>
</nav>
