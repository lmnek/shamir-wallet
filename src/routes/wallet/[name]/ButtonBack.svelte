<script lang="ts">
    import { goto } from "$app/navigation";
    import { faChevronLeft } from "@fortawesome/free-solid-svg-icons";
    import Fa from "svelte-fa";
    export let url: string;
    $: parentUrl = url.match("(/wallet/.*?)/")![1];
</script>

<button
    class="btn self-start variant-ghost-secondary my-5 mx-1"
    on:click={() => goto(parentUrl)}
>
    <Fa icon={faChevronLeft} size="1x" />
    <p>Back</p>
</button>
