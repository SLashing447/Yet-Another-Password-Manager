<script>
  import Inputs from "./utils/Inputs.svelte";
</script>

<div class="wrapper">
  <div class="card">
    <h3>YPM Manager</h3>
    <Inputs type={1} />
  </div>
</div>

<style>
  .wrapper {
    /* background-color: grey; */
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;

    /* background-color: rgb(70, 77, 84); */
  }
  .card {
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    align-items: center;
    justify-content: center;
    background-color: rgba(60, 59, 59, 0.482);
  }
</style>
