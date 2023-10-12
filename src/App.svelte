<script lang="ts">
  import VGutter from "./lib/web/componets/VGutter.svelte";
  import Header from "./lib/web/componets/Header.svelte";
  import Operations from "./lib/web/componets/Operations.svelte";
  import Recipe from "./lib/web/componets/Recipe.svelte";
  import IO from "./lib/web/componets/IO.svelte";
</script>

<main>
  <!-- <Header/> -->
  <div class="content-wrapper">
    <Operations/>
    <VGutter/>
    <Recipe/>
    <VGutter/>
    <IO/>
  </div>
</main>


<style>
  .content-wrapper {
    display: grid;
    grid-template-columns: calc(20% - 2px) 4px calc(30% - 4px) 4px calc(50% - 2px);
    height: 100%;
    /* height: calc(100% - 30px); */
  }

  main {
    background-color: var(--primary-background-color);
  }
  
</style>

