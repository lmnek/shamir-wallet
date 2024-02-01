<script lang="ts">
    import { invoke } from "@tauri-apps/api";
    import type { TransactionData } from "../../../../Types";
    import { page } from "$app/stores";
    import ButtonBack from "../ButtonBack.svelte";
    // HACK: used regex to parse URL, is there a cleaner way in Svelte?
    let walletName = $page.url.pathname.match("\/wallet\/(.*?)\/")![1] 

    let transactions = invoke<TransactionData[]>("get_transactions", { name: walletName });
</script>

{#await transactions}
    <p>Loading...</p>
{:then txs}
    <ButtonBack url={ $page.url.pathname }/>
    <div class="table-container m-3">
        <table class="table table-hover">
            <thead>
                <tr>
                    <th>Send/Receiced</th>
                    <th>Transaction ID</th>
                    <th  class="text-right">Fee</th>
                    <th  class="text-right">Amount</th>
                </tr>
            </thead>
            <tbody>
                {#each txs as tx}
                    <!-- TODO: idea: link to the blockchain explorer tx -->
                    <tr>
                        <td>{(tx.sent === 0) ? "Received" : "Sent"}</td>
                        <td>{tx.tx_id}</td>
                        <!-- TODO: fee amount -->
                        <td class="text-right">100</td> 
                        <td class="text-right">{(tx.sent === 0) ? tx.received : tx.sent}</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{:catch error}
    <p>Failed to load transactions</p>
    <p>{error}</p>
{/await}
