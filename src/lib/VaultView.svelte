<script lang="ts">
    import { path, routeTo } from "../scripts/utils";
    import Card from "./Card.svelte";
    import CardView from "./CardView.svelte";
    import Router from "./Router.svelte";

    let items = Array.from({ length: 10 }, (_, i) => i);

    function onCardClick(name: string) {
        path.update((arr) => {
            arr.push(name); // mutates, but we still return it to notify subscribers
            return [...arr]; // better to return a new array copy
        });
    }

    function addConn() {
        if ($path.length === 2) {
            routeTo("-1");
            routeTo("add-card");
        } else {
            routeTo("add-card");
        }
    }
</script>

<div class="page">
    <h2>Vault XYZ</h2>
    <div class="vault-controls">
        <div>
            <button onclick={addConn}>{"📎"}</button>
            <button>{"🔐"}</button>
            <button>{"💡"}</button>
        </div>
    </div>

    <Router />

    {#if $path.length === 1}
        <div class="cards">
            {#each items as i}
                <div>
                    <Card
                        onClick={() => onCardClick("steam")}
                        name="Steam"
                        img="src/assets/svelte.svg"
                        dsc="This is Steam Profile"
                    />
                </div>
            {/each}
        </div>
    {/if}

    {#if $path.length > 1}
        <div class="card-view">
            <CardView />
        </div>
    {/if}
    <div class="footer">
        <span>Last Accessed : Oct 3,2024</span>
        <span>Created At: Oct 1,2025</span>
    </div>
</div>

<style>
    .footer {
        margin-top: auto;
        display: flex;
        justify-content: space-between;
    }
    .footer > span {
        font-family: monospace;
        font-size: 11px;
        color: grey;
    }

    .vault-controls {
        display: flex;
        justify-content: space-between;
        margin-bottom: 10px;
    }
    .vault-controls > div > button {
        outline: none !important;
        cursor: pointer;
        border: none !important;
        background-color: rgba(142, 95, 190, 0.334);
        padding: 0.2rem 1rem;
        border-radius: 5px;
    }
    .vault-controls > div > button:hover {
        background-color: rgba(177, 118, 236, 0.334);
    }
    .page {
        /* display: grid; */
        max-width: 40rem;
        flex: 1;
         /*border: 1px solid lime; */
        padding: 0.5rem 0.5rem 0 0.5rem;
        display: flex;
        flex-direction: column;
    }
    .page > h2 {
        margin-bottom: 0.3rem;
    }
    .cards {
        display: flex;
        flex-wrap: wrap;
        padding: 0.3rem;
        position: relative;
        gap: 0.5rem;
    }
</style>
