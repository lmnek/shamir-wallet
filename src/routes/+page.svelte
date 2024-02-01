<script lang="ts">
    import { goto } from '$app/navigation';
    async function handleClick() {
        goto("/login")
    }
</script>

<div class="container h-full mx-auto flex justify-center mt-36">
	<div class="space-y-10 flex flex-col items-center">
		<h2 class="h2 text-center">Welcome to the Shamir Wallet</h2>
        <button class="btn variant-filled-primary" on:click={handleClick}>
            Continue
        </button>
	</div>
</div>

