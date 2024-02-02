<script lang="ts">
    import { goto } from "$app/navigation";
    async function handleClick() {
        goto("/login");
    }
</script>

<div class="container mx-auto">
    <div class="space-y-10 flex flex-col items-center mt-24">
        <section class="img-bg" />
        <h2 class="h2 text-center">Welcome to the Shamir Wallet</h2>
        <button
            class="btn font-bold variant-filled-primary"
            on:click={handleClick}
        >
            Continue
        </button>
    </div>
</div>

<style lang="postcss">
    .img-bg {
        @apply w-96 h-96;
    }
    .img-bg {
        @apply absolute z-[-1] rounded-full blur-[50px] transition-all;
        animation:
            pulse 5s cubic-bezier(0, 0, 0, 0.5) infinite,
            glow 5s linear infinite;
    }
    @keyframes glow {
        0% {
            @apply bg-primary-400/50;
        }
        33% {
            @apply bg-secondary-400/50;
        }
        66% {
            @apply bg-tertiary-400/50;
        }
        100% {
            @apply bg-primary-400/50;
        }
    }
    @keyframes pulse {
        50% {
            transform: scale(1.5);
        }
    }
</style>
