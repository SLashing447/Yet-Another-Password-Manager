<script lang="ts">
    import { onMount } from "svelte";
    import { routeBack, path, selected_vault } from "../scripts/utils";
    import Auth from "./Auth.svelte";
    import Card from "./Card.svelte";
    import CardView from "./CardView.svelte";
    // import Notes from "./Notes.svelte";
    import Search from "./Search.svelte";
    import Vaults from "./Vaults.svelte";
    import VaultView from "./VaultView.svelte";
    import AddCard from "./AddCard.svelte";
    // import { writable, type Writable } from "svelte/store";
    import { invoke } from "@tauri-apps/api/core";
    let summary: string = $state("");

    invoke("note_summary").then((data) => (summary = data as string));

    // console.log("Summary : ", summary);

    function handleKey(e: KeyboardEvent) {
        if (e.key === "Escape") {
            routeBack();
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
    const vaults: Vault[] = [
        { id: "1", name: "Game Store", dsc: "Gaming Vault" },
        {
            id: "2",
            name: "Photo Locker",
            dsc: "Store personal photos securely",
        },
        { id: "3", name: "Music Box", dsc: "Vault for playlists and tracks" },
        { id: "4", name: "Dev Hub", dsc: "Vault for code snippets and APIs" },
        {
            id: "5",
            name: "Finance Safe",
            dsc: "Vault for banking and expense data",
        },
    ];

    // function onCardClick
</script>

<main>
    <div class="wrapper">
        <Search />
        {summary}

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
