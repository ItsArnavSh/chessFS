<script>
    import { onMount } from 'svelte';
  
    let count = 0;
    let data;
    async function fetchData() {
    let address = 'http://localhost:8000/'+count
      const response = await fetch(address);
      if (!response.ok) {
        console.error('Failed to fetch data:', response.status, response.statusText);
        return;
      }
      data = await response.json();
      data = JSON.parse(data);
    }

    function increment() {
      count += 1;
    }

    function handleClick() {
      fetchData();
      increment();
    }
</script>

<button on:click={handleClick}>Fetch Data</button>

{#if data}
    <p>{data.message}</p>
{:else}
    <p>Loading...</p>
{/if}
