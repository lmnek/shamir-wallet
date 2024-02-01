<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import type { TransactionData } from "../../../../Types";
    import { page } from "$app/stores";
    import { open } from "@tauri-apps/api/shell";
    import ButtonBack from "../ButtonBack.svelte";
    import Fa from "svelte-fa";
    import { faArrowUpRightFromSquare } from "@fortawesome/free-solid-svg-icons";
    // HACK: used regex to parse URL, is there a cleaner way in Svelte? (also in Send)
    let walletName = $page.url.pathname.match("/wallet/(.*?)/")![1];

    let transactions = invoke<TransactionData[]>("get_transactions", {
        name: walletName,
    });

    async function openTransaction(tx_id: string) {
        let isTestnet = await invoke("is_testnet");
        await open(
            `https://live.blockcypher.com${
                isTestnet ? "/btc-testnet" : ""
            }/tx/${tx_id}/`,
        );
    }
</script>

{#await transactions}
    <p>Loading...</p>
{:then txs}
    <ButtonBack url={$page.url.pathname} />
    <div class="table-container m-3">
        <table class="table table-hover">
            <thead>
                <tr>
                    <th>Type</th>
                    <th>Transaction ID</th>
                    <th class="text-right">Fee</th>
                    <th class="text-right">Amount</th>
                </tr>
            </thead>
            <tbody>
                {#each txs as tx}
                    <tr>
                        <td>{tx.sent === 0 ? "Received" : "Sent"}</td>
                        <td class="flex space-x-2 items-center">
                            <div>{tx.tx_id}</div>
                            <button on:click={() => openTransaction(tx.tx_id)}>
                                <Fa icon={faArrowUpRightFromSquare} />
                            </button>
                        </td>
                        <td class="text-right"
                            >{tx.fee === null ? "-" : tx.fee}</td
                        >
                        <td class="text-right"
                            >{tx.sent === 0 ? tx.received : tx.sent}</td
                        >
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{:catch error}
    <p>Failed to load transactions</p>
    <p>{error}</p>
{/await}
