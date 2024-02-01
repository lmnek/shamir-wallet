<script lang="ts">
    import { goto } from '$app/navigation';
    async function handleClick() {
        goto("/login")
    }
</script>

<div class="container mx-auto">
	<div class="space-y-10 flex flex-col items-center mt-44">
		<h2 class="h2 text-center">Welcome to the Shamir Wallet</h2>
        <button class="btn font-bold variant-filled-primary" on:click={handleClick}>
            Continue
        </button>
	</div>
</div>

