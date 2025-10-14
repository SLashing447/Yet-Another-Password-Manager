<script lang="ts">
  import { selected_vault } from "../scripts/utils";
  import Card from "./Card.svelte";

  interface Vault {
    id: string;
    name: string;
    dsc: string;
  }

  interface Props {
    vaults: Vault[];
  }

  let { vaults }: Props = $props();

  let unlcoked: number[] = $state([]);
  let items = Array.from({ length: 5 }, (_, i) => i);
  let unlockAtmpt: number = $state(-1);

  // @state name: number = 0;

  // let active: string = $state("");
</script>

<div class="wrapper">
  <h2>My Vaults</h2>
  <div class="vaults">
    <Card
      selected={$selected_vault === "0"}
      name="Default"
      img="src/assets/svelte.svg"
      dsc="Default Vault"
      onClick={() => selected_vault.set("0")}
    />
    {#each vaults as v, k}
      <div class="card">
        {#if unlockAtmpt !== k}
          <Card
            dsc={v.dsc}
            img={"src/assets/svelte.svg"}
            name={v.name}
            selected={$selected_vault === v.id}
            onClick={() => {
              if (unlockAtmpt !== k) unlockAtmpt = -1;
              selected_vault.set(v.id);
            }}
          />
        {:else}
          <div class="lock-inp">
            🔑
            <input type="password" placeholder={`${v.name} Password`} />
          </div>
        {/if}
        <button
          onclick={() => {
            // console.log("hi : ", k);
            if (unlockAtmpt === k) unlockAtmpt = -1;
            unlockAtmpt = k;
          }}
          style={unlockAtmpt === k ? "opacity:1" : ""}
          class="status"
        >
          {#if unlockAtmpt === k}
            {"🔐"}
          {:else}
            {unlcoked.find((d) => d === k) ? "🔓" : "🔐"}
          {/if}
        </button>
      </div>
    {/each}
  </div>
  <div class="toolbar">
    <button> Add Vault</button>
    <button>{"📛"}</button>
  </div>
</div>

<style>
  .wrapper {
    padding: 0.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    /* justify-content: center; */
  }
  input {
    background-color: transparent !important;
    outline: none !important;
    width: 100%;
    border: none !important;
  }
  input::placeholder {
    font-weight: bold;
  }
  .lock-inp {
    display: flex;
    gap: 0.5rem;
    border: 3px solid var(--acc2);
    border-radius: 5px;
    width: 100%;
    padding: 0.3rem 1rem;
  }

  .vaults {
    scrollbar-width: none;
    display: flex;
    flex-direction: column;
    /* align-items: stretch; */
    gap: 0.6rem;
    overflow-y: auto;
    /* border: 1px solid red; */
    /* height: 100%; */
  }

  .card {
    display: flex;
    /* width: 300px; */
    /* justify-content: center; */
    /* align-items: center; */
    flex: 1;
    /* justify-content: space-between; */
    /* border: 1px solid lime; */
    width: 250px;
    gap: 0.3rem;
    /* width: 100%; */
  }

  button {
    outline: none !important;
    border: none !important;
  }

  .status {
    cursor: pointer;
    opacity: 0;
    font-size: large;
    /* border: 1px solid red; */
    background-color: transparent;
  }
  .card:hover > .status {
    opacity: 1;
  }
  .toolbar {
    margin-top: auto;
  }
  .toolbar > button {
    outline: none !important;
    border: none !important;
    background-color: rgba(162, 102, 162, 0.557);
    padding: 0.2rem 1rem;
    cursor: pointer;
    border-radius: 4px;
  }
</style>
