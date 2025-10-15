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
    let passInp: string = $state("");
    let passInpField: HTMLInputElement;

    let unlockAtmpt: number = $state(1);
    let createVault: boolean = $state(false);

    // @state name: number = 0;
    //     <span>{"＋"}</span>
    // let active: string = $state("");
</script>

<div class="wrapper">
    <div class="header">
        <h2>My Vaults</h2>

        <button onclick={() => (createVault = !createVault)} title="add vault">
            <span class={createVault ? "plus cross" : "plus"}></span>
        </button>
    </div>
    <div class="vaults">
        {#if !createVault}
            <Card
                selected={$selected_vault === "0"}
                name="Default"
                img="src/assets/svelte.svg"
                dsc="Default Vault"
                onClick={() => selected_vault.set("0")}
            />
            {#if unlockAtmpt === -1}
                {#each vaults as v, k}
                    <div class="card">
                        <Card
                            dsc={v.dsc}
                            img={"src/assets/svelte.svg"}
                            name={v.name}
                            selected={$selected_vault === v.id}
                            onClick={() => {
                                if (!unlcoked.includes(k)) {
                                    console.log("Opening attempt");
                                    unlockAtmpt = k;
                                    // selected_vault.set(v.id);
                                }
                            }}
                        />
                    </div>
                {/each}
            {:else}
                <div class="unlock">
                    <h3>
                        <button onclick={() => (unlockAtmpt = -1)}>
                            ❌
                        </button>Unclock Vault
                    </h3>
                    <div class="lock-inp">
                        <input type="password" placeholder="Enter Password" />
                    </div>
                </div>
            {/if}
        {:else}
            <div class="create-vault">
                <h3>Create Vault</h3>
                <form class="form">
                    <div class="input">
                        <span> 💾 </span>
                        <input
                            required
                            name="name"
                            type="text"
                            placeholder="Vault Name"
                        />
                    </div>
                    <div class="input">
                        <span> 📝 </span>
                        <input
                            type="text"
                            name="dsc"
                            placeholder="Description (Optional)"
                        />
                    </div>

                    <div class="input">
                        <span>🔑</span>
                        <input
                            required
                            type="password"
                            name="password"
                            placeholder="Password"
                        />
                    </div>
                    <div class="input">
                        <span>🔑</span>
                        <input
                            required
                            type="password"
                            placeholder="Confirm Password"
                        />
                    </div>
                    <div class="input btn">
                        <button>Add</button>
                    </div>
                </form>
            </div>
        {/if}
    </div>
</div>

<style>
    .wrapper {
        padding: 0.5rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        min-width: 18rem;
        /*border: 1px solid red;*/
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
    .unlock {
        /*border: 1px solid red;*/
        /*padding: 0.4rem 1rem;*/
        margin-top: 0.2rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;

        border-radius: 5px;
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
    .unlock > h3 > button {
        cursor: pointer;
    }
    .unlock > h3 {
        display: flex;
        background-color: #482b4861;
        width: 100%;
        border-radius: 6px;
        padding: 0.2rem 1rem;
        gap: 0.5rem;
        /*justify-content: center;*/
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

    .header {
        /*border: 1px solid white;*/
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .header > button:hover {
        background-color: rgba(162, 102, 162, 0.157);
    }
    button {
        outline: none !important;
        border: none !important;
        background-color: transparent;
    }
    .header > button {
        font-size: 18px;
        /*padding: 0.1rem 0.4rem;*/
        cursor: pointer;
        font-weight: bold;
        border-radius: 4px;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .plus {
        width: 40px;
        height: 40px;
        position: relative;
        /*border: 1px solid red;*/
        display: inline-block;
        transform-origin: center;
        /* optional animation */
        transition: transform 0.15s ease;
    }
    .plus::before,
    .plus::after {
        content: "";
        position: absolute;
        left: 50%;
        top: 50%;
        background: lightgrey;
        transform: translate(-50%, -50%);
        border-radius: 2px;
    }
    .plus::before {
        width: 35%;
        height: 2px;
    } /* horizontal bar */
    .plus::after {
        width: 2px;
        height: 35%;
    } /* vertical bar */

    .create-vault > form {
        width: 100%;
    }

    /* make it a cross */
    .plus.cross {
        transform: rotate(45deg);
    }
    .input.btn > button {
        cursor: pointer;
        width: 100%;
    }
    .input {
        display: flex;
        gap: 0.6rem;
        /* background-color: ; */
        box-shadow: inset 0 0px 0 1.4px var(--acc1);
        padding: 0.2rem 1rem;
        border-radius: 4px;
    }
    .input > input::placeholder {
        font-weight: bold;
    }

    .form {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        width: 80%;
        margin: 0.5rem 0;
        align-self: center;
        background-color: rgba(87, 70, 113, 0.217);
        padding: 0.5rem;
        border-radius: 5px;
    }
</style>
