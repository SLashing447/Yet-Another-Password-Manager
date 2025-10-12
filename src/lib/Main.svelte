<script>
  import Auth from "./Auth.svelte";
  import Card from "./Card.svelte";
  import Search from "./Search.svelte";
</script>

<main>
  <div class="wrapper">
    <Search />
    <div class="page">
      <Card />
      <Card />
      <Card />
      <Card />
      <Card />
      <Card />
      <Card />
    </div>
  </div>
</main>

<style>
  main {
    display: flex;
    height: 100vh;
    justify-content: center;
  }
  .page {
    display: grid;
    grid-template-columns: repeat(3, 1fr); /* 3 equal columns */
    gap: 0.3rem;
    padding: 1rem 0.2rem ;
  }
  .wrapper {
    padding: 1rem 0.3rem;
    height: 100%;
    position: relative;
    /* width: 10rem; */
    /* background-color: red; */
  }
</style>
