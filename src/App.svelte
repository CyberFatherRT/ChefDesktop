<script lang="ts">
  import Gutter from "./lib/web/componets/Gutter.svelte";
  import Header from "./lib/web/componets/Header.svelte";
</script>

<main>
  <Header/>
  <div class="content-wrapper">
    <div class="box"></div>
    <Gutter/>
    <div class="box"></div>
    <Gutter/>
    <div class="box"></div>
  </div>
</main>


<style>
  .content-wrapper {
    display: grid;
    grid-template-columns: calc(20% - 2px) 4px calc(30% - 4px) 4px calc(50% - 2px);
    height: calc(100% - 30px);
  }

  main {
    background-color: var(--primary-background-color);
  }
  
</style>

