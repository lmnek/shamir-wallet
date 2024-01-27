<script lang="ts">
    import { invoke } from '@tauri-apps/api';
    
    let words = "";

    async function handleClick() {
        words = await invoke("cw")
    }
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-10 text-center flex flex-col items-center">
		<h2 class="h2">Welcome to Shamir.</h2>
        <a href="/login">Login</a>
        <button class="btn variant-filled" on:click={handleClick}>
            Create wallet
        </button>
        <p>{words}</p>
	</div>
</div>

