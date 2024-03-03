<script>
    export let block;
</script>
{#if block==0}
<button class = "bg-green-400">
    Button
    </button>
{:else}
<button class = "bg-yellow-800">
    Button
    </button>
{/if}
