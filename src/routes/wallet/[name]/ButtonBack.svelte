<script lang="ts">
    import { goto } from "$app/navigation";
    export let url: string
    $: parentUrl = url.match("(\/wallet\/.*?)\/")![1]
</script>

<button class="btn self-start variant-ghost-secondary my-3 mx-1"
    on:click={() => goto(parentUrl)}>Back</button>
