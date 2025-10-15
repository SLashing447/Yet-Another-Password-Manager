<script lang="ts">
    import { onMount } from "svelte";
    import { path, routeTo, selected_vault } from "../scripts/utils";
    import Auth from "./Auth.svelte";
    import Card from "./Card.svelte";
    import CardView from "./CardView.svelte";
    // import Notes from "./Notes.svelte";
    import Search from "./Search.svelte";
    import Vaults from "./Vaults.svelte";
    import VaultView from "./VaultView.svelte";
    import AddCard from "./AddCard.svelte";
    // import { writable, type Writable } from "svelte/store";
    // import { invoke } from "@tauri-apps/api/core";
    // let summary: string = $state("");

    // invoke("note_summary").then((data) => (summary = data as string));

    // console.log("Summary : ", summary);

    function handleKey(e: KeyboardEvent) {
        if (e.key === "Escape") {
            routeTo("-1");
        }
    }

    onMount(() => {
        window.addEventListener("keydown", handleKey);
        return () => window.removeEventListener("keydown", handleKey);
    });

    interface Vault {
        id: string;
        name: string;
        dsc: string;
    }
    const vaults: Vault[] = [];

    // function onCardClick
</script>

<main>
    <div class="wrapper">
        <Search />

        {#if $path[1] === "add-card"}
            <AddCard />
        {:else}
            <VaultView />
        {/if}
        <Vaults {vaults} />
    </div>
</main>

<style>
    main {
        display: flex;
        height: 100vh;
        justify-content: center;
    }

    .wrapper {
        padding: 1rem 0.3rem;
        height: 100%;
        position: relative;
        display: flex;
        width: 70rem;
        /* align-items: center; */
    }
</style>
