<script lang="ts">
  import { onMount } from "svelte";
  import Operator from "../../scripts/API";
  import { isAuthenticated } from "../../scripts/utils";

  let focus: boolean = $state(false);
  let isMaximized: boolean = $state(false);
  let srchEl: HTMLInputElement | null = $state(null);
  let srchTxt: string = $state("");

  const minimize = async () => await Operator.windowFn("Min");
  const close = async () => await Operator.windowFn("Close");
  const maxm = async () => {
    if (isMaximized) {
      await Operator.windowFn("UnMax");
    } else {
      await Operator.windowFn("Max");
    }
    isMaximized = !isMaximized;
  };

  function handleKey(e: KeyboardEvent) {
    if (e.ctrlKey && e.key === "f") {
      e.preventDefault();
      if (srchEl) srchEl.focus();
      focus = true;
    }
  }

  onMount(() => {
    window.addEventListener("keydown", handleKey);
    return () => window.removeEventListener("keydown", handleKey);
  });

  // import { appWindow } from "@tauri-apps/api/window";
</script>

<div
  data-tauri-drag-region={true}
  class={`titlebar ${$isAuthenticated ? "" : "auth"}`}
>
  <div class="title" title="Yet Another Password Manager">
    <span>YAPM</span>
  </div>
  {#if $isAuthenticated}
    <button
      onfocus={() => (focus = true)}
      onblur={() => (focus = false)}
      title="Search"
      class={`search ${focus ? "foc" : ""}`}
    >
      <span class="code"> Ctrl F</span>

      <div class="input">
        <input
          bind:this={srchEl}
          bind:value={srchTxt}
          onfocus={() => (focus = true)}
          onblur={() => (focus = false)}
          type="text"
          placeholder="🔍 Search"
        />
      </div>
    </button>
  {/if}
  <div class="window-controls">
    <button onclick={minimize}>–</button>
    <button onclick={maxm} style="font-size: 12px;">
      {#if !isMaximized}
        🗖
      {:else}
        🗗
      {/if}
    </button>

    <button onclick={close}>×</button>
  </div>
</div>

<style>
  .titlebar.auth {
    background: #121212;
    padding: 0.2rem 1rem;
    border-radius: 5px;
    margin-bottom: 0.7rem;
  }
  .titlebar {
    margin: 0.4rem;
    margin-bottom: 0;
    /*margin-bottom: 1rem;*/
    /*height: 28px;*/
    /*padding: 0.3rem 0.5rem;*/
    /*border-radius: 5px;*/
    color: #eee;
    /*padding-bottom: 0.3rem;*/
    /*border-bottom: 1px solid darkgrey;*/
    display: flex;
    justify-content: space-between;
    /*justify-content: flex-end;*/
    align-items: center;
    -webkit-app-region: drag;
    transition: 0.15s all ease;
    cursor: default !important;
    user-select: none !important;
    /*border: 1px solid red;*/
  }

  .search {
    /*border: 1px solid red;*/
    width: 30%;
    border-radius: 6px;
    border: 2.3px solid rgb(76, 64, 89);
    padding: 0.2rem 1rem;
    background-color: transparent;
    outline: none !important;
    display: flex;
    position: relative;
  }
  .search > .input {
    flex: 1;
  }
  .search.foc > .code {
    display: none;
  }
  .search > .code {
    font-weight: bold;
    font-family: monospace;
    font-size: 12px;
    /*background-color: purple;*/
    padding: 0.1rem 0.4rem;
    background-color: rgb(59, 30, 92);
    position: absolute;
    border-radius: 5px;
    top: 50%;
    transform: translateY(-50%);
    /*box-shadow: inset 0 -2px 0 rgb(189, 112, 189);*/
  }

  .search:hover {
    background-color: rgba(30, 30, 30, 0.5);
  }
  .search.foc {
    background-color: rgba(30, 30, 30, 0.5);
    border-color: rgb(92, 44, 145);
  }
  .search > .input > input::placeholder {
    font-weight: bold;
  }
  .search > .input > input {
    text-align: center;
    width: 100%;
    border: none !important;
    outline: none !important;
    background-color: transparent !important;
  }

  .title {
    /*border: 1px solid red;*/
    font-weight: bold;
    margin-left: 1rem;
  }
  .title > span {
    font-family: monospace;
    color: lightgrey;
  }

  .titlebar:hover > .window-controls {
    display: flex;
  }

  .window-controls {
    display: flex;
    /*border: 1px solid red;*/
    /*margin-right: 0.4rem;*/
    gap: 0.1rem;
  }
  .window-controls button {
    -webkit-app-region: no-drag;
    background: transparent;
    border: none;
    color: #eee;
    padding: 0 12px;
    font-size: 16px;
    font-weight: bold;
    border-radius: 5px;
    /*cursor: pointer;*/
  }

  .window-controls button:hover {
    background-color: #907da05c;
  }
</style>
