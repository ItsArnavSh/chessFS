<script>
    import { onMount } from "svelte";
    let data;
    let index = 0;
    let iterableArray = 
    [
        0,1,0,1,0,1,0,1,
        1,0,1,0,1,0,1,0,
        0,1,0,1,0,1,0,1,
        1,0,1,0,1,0,1,0,
        0,1,0,1,0,1,0,1,
        1,0,1,0,1,0,1,0,
        0,1,0,1,0,1,0,1,
        1,0,1,0,1,0,1,0
    ]
    //gameData is the pgn 
    async function fetchData(ind=' ') {
    let address = 'http://localhost:8000/'+ind;
      const response = await fetch(address);
      if (!response.ok) {
        console.error('Failed to fetch data:', response.status, response.statusText);
        return;
      }
      data = await response.json();
      data = JSON.parse(data);
      
    }
    onMount(async () => {
        let address = 'http://localhost:8000/startmap';
        const response = await fetch(address);
        if (!response.ok) {
            console.error('Failed to fetch data:', response.status, response.statusText);
            return;
        }
        data = await response.json();
        data = JSON.parse(data);
        return data
    })


</script>
{#if data!=undefined}
<div>{data.message}</div>
{/if}
<div class = "flex flex-col items-center">
    <h1>Hello</h1>
<div class = "grid grid-cols-8 grid-rows-8 gap-0 justify-center w-[40%] border-8 border-black ">
{#each iterableArray as item,index}
    
    <button id = {index} class = "aspect-1 p-0 m-0 {item % 2 === 0 ? 'bg-amber-50' : 'bg-lime-600'} border border-black" on:click={()=>fetchData(index)}>
        
    </button>
{/each}
</div>
</div>