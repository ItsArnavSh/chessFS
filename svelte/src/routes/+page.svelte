<script>
    import { onMount } from 'svelte';
  
    let data;
  
    onMount(async () => {
      const response = await fetch('http://localhost:8000/api/data');
      if (!response.ok) {
        console.error('Failed to fetch data:', response.status, response.statusText);
        return;
      }
      data = await response.json();
      data = JSON.parse(data);
    });
  </script>
  
  {#if data}
    <p>{data.message}</p>
  {:else}
    <p>Loading...</p>
  {/if}