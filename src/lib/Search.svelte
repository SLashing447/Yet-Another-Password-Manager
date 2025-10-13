<div class="wrapper">
  <div class="search">
    <span class="tag">🔎</span>
    <input placeholder="Search Anywhere" type="text" />
  </div>
</div>

<style>
  .wrapper {
    padding: 0.5rem;

    width: 15rem;
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
    font-size: 17px;
    display: flex;
    padding: 0.4rem 1rem;
    display: flex;
    gap: 0.5rem;
  }
</style>
