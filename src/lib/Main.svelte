<script lang="ts">
    import { onMount } from "svelte";
    import { isAuthenticated, path, routeTo } from "../scripts/utils";
    import Search from "./Search.svelte";
    import Vaults from "./Vaults.svelte";
    import VaultView from "./VaultView.svelte";
    import AddCard from "./AddCard.svelte";
    import Auth from "./Auth.svelte";
    import Titlebar from "./utils/Titlebar.svelte";

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
    const vaults: Vault[] = [
        {
            dsc: "Hello world",
            id: "123ere2wfuihf",
            name: "Gaming",
        },
    ];
</script>

<main>
    <!-- <Titlebar /> -->

    <div class="wrapper">
        {#if $isAuthenticated}
            <Search />

            {#if $path[1] === "add-card"}
                <AddCard />
            {:else}
                <VaultView />
            {/if}
            <!-- <Store /> -->
            <Vaults {vaults} />
        {:else}
            <Auth />
        {/if}
    </div>
</main>

<style>
    main {
        display: flex;
        height: 100vh;
        flex-direction: column;
        justify-content: center;
    }

    .wrapper {
        padding: 1rem 0.3rem;
        height: 100%;
        position: relative;
        display: flex;
        width: 70rem;
        align-self: center;
        /* align-items: center; */
    }

    @media (max-width: 1120px) {
        .wrapper {
            width: 100%;
            /*border: 1px solid red;*/
        }
    }
</style>
