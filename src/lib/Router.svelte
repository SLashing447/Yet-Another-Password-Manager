<script lang="ts">
    import { path, routeTo } from "../scripts/utils";

    function onRoute(i: number) {
        if (i === 0 && $path.length > 1) routeTo("-1");
    }
</script>

<div class="wrapper">
    {#each $path as ro, i}
        <button onclick={() => onRoute(i)}>
            <img src="src/assets/svelte.svg" alt="" />
            <span> {ro}</span>
        </button>
        {#if $path.length > 1 && $path.length - 1 !== i}
            {"> "}
        {/if}
    {/each}
</div>

<style>
    .wrapper {
        /* border: 1px solid red; */
        padding: 0.05rem 0.5rem;

        border-radius: 4px;
    }
    button:hover {
        background-color: rgba(143, 77, 163, 0.505);
    }
    button {
        background-color: rgba(143, 77, 163, 0.263);
        border: none !important;
        outline: none !important;
        padding: 0 1rem;
        border-radius: 10px;
        cursor: pointer;
        /* background-color: transparent !important; */
    }
    button > img {
        width: 10px;
    }
    button:hover {
        text-decoration: underline;
    }
</style>
