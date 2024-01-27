<script lang="ts">
    import { goto } from "$app/navigation";
    export let data: { name: string, balance: number }
    //export let balance: number
    //export let name: string
</script>

<div>{data.name}: {data.balance}</div>
<button on:click={() => {
    goto('/')
}}>go back</button>
