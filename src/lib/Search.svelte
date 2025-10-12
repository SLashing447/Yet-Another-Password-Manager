<div class="wrapper">
  <div class="search">
    <input placeholder="Search" type="text" />
  </div>
</div>

<style>
  .wrapper {
    width: 100%;
    position: sticky;
    backdrop-filter: blur(30px);
    top: 0;
  }
  input {
    width: 100%;
    background-color: transparent !important;
    outline: none !important;
    border: none !important;
  }
  input::placeholder {
    color: white;
  }
  .search {
    border: 2px solid var(--acc2);
    border-radius: 3px;
    font-size: 18px;
    display: flex;
    padding: 0.4rem 1rem;
  }
</style>
