<script lang="ts">
    import Cloud from "./Cloud.svelte";
    import Magic from "./Magic.svelte";
    import Profile from "./Profile.svelte";
    import Settings from "./Settings.svelte";

    let page: number = $state(0);
    let options = ["🪪", "✨", "☁️", "⚙️"];
</script>

<div class="wrapper">
    <div class="tool-bar">
        {#each options as opt, i}
            <button onclick={() => (page = i)} class={page === i ? "sel" : ""}>
                {opt}
            </button>
        {/each}
    </div>
    <div class="body">
        <!-- <h2>Welcome</h2> -->
        {#if page === 0}
            <Profile />
        {:else if page === 1}
            <Magic />
        {:else if page === 2}
            <Cloud />
        {:else}
            <Settings />
        {/if}
    </div>
</div>

<style>
    .wrapper {
        height: 100%;
        /*border: 1px solid red;*/
        padding: 0.5rem 0.5rem 0 0.5rem;
        min-width: 15rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
    .body {
        flex: 1;
    }
    .tool-bar {
        display: flex;
        /*flex-direction: column;*/
        gap: 0.3rem;
    }
    button.sel {
        /*border: 2px solid purple;*/
        /* purple;*/
        /*background-color: purple;*/

        box-shadow: inset 0 0 0 2px rgba(138, 55, 180, 0.82);

        background-color: rgba(170, 74, 170, 0.3);
    }
    button:hover {
        background-color: rgba(170, 74, 170, 0.3);
    }
    button {
        outline: none !important;
        border: none !important;
        padding: 0.2rem 0.5rem;
        font-size: 14px;
        cursor: default !important;
        border-radius: 3px;
        background-color: rgba(111, 82, 111, 0.45);
    }
</style>
