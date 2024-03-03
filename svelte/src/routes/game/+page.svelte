<script>
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
    let coord = Array.from({ length: 64 }, (_, i) => i + 1);

</script>
<div class = "flex flex-col items-center">
    <h1>Hello</h1>
<div class = "grid grid-cols-8 grid-rows-8 gap-0 justify-center w-[40%] border-8 border-black ">
{#each iterableArray as item,index}
    
    <button id = {index} class = "aspect-1 p-0 m-0 {item % 2 === 0 ? 'bg-amber-50' : 'bg-lime-600'} border border-black" on:click={()=>console.log("Clicked ",index)}></button>
{/each}
</div>
</div>