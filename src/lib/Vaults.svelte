<script lang="ts">
    import Operator, { DataType } from "../scripts/API";
    import type { FormProps, VaultSchema } from "../scripts/types";
    import { selected_vault } from "../scripts/utils";
    import Store from "./Store.svelte";
    import Card from "./utils/Card.svelte";
    import Form from "./utils/Form.svelte";

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

    let unlockAtmpt: number = $state(-1);
    let createVault: boolean = $state(false);

    function onCross() {
        if (unlockAtmpt !== -1) {
            unlockAtmpt = -1;
            return;
        }

        createVault = !createVault;
    }

    async function onCreateVault(data: VaultSchema) {
        await Operator.post(DataType.Vault, data);
    }
    async function onUnlockVault({ password }: any) {
        const id = vaults[unlockAtmpt].id;
        await Operator.unlock(DataType.Vault, password, id);
    }

    const INPUTS: FormProps[] = [
        {
            name: "💾/name",
            placeholder: "Vault Name",
            required: true,
        },
        {
            name: "📝/dsc",
            placeholder: "Description (Optional)",
        },

        {
            name: "🔑/Password",
            placeholder: "Vault Password (Optional)",
            stMeter: true,
            type: "pass",
        },
        {
            name: "🔑/Confirm Password",
            type: "cnfm-pass",
            placeholder: "Confirm Password (Optional)",
            required: false,
        },
    ];
</script>

<div class="wrapper">
    <Store />
    <div class="header">
        <h2>My Vaults</h2>

        <button
            onclick={onCross}
            title={createVault || unlockAtmpt !== -1
                ? "Go Back"
                : "Create Vault"}
        >
            <span
                class={createVault || unlockAtmpt !== -1
                    ? "plus cross"
                    : "plus"}
            ></span>
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
                    <h3>Unclock {vaults[unlockAtmpt].name}</h3>
                    <Form
                        inputs={[
                            {
                                name: "password",
                                required: true,
                                placeholder: "Vault Key",
                                type: "pass",
                            },
                        ]}
                        onSubmit={onUnlockVault}
                        SubmitBtn="Unlock"
                    />
                </div>
            {/if}
        {:else}
            <div class="create-vault">
                <h3>Create Vault</h3>
                <Form
                    inputs={INPUTS}
                    SubmitBtn="Create"
                    onSubmit={onCreateVault}
                />
            </div>
        {/if}
    </div>
</div>

<style>
    .wrapper {
        padding: 0.5rem;
        display: flex;
        flex-direction: column;
        /*gap: 1rem;*/
        min-width: 18rem;
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

    .unlock > h3 {
        display: flex;
        background-color: #482b4861;
        width: 100%;
        border-radius: 6px;
        padding: 0.2rem 1rem;
        gap: 0.5rem;
        /*justify-content: center;*/
    }
    .create-vault {
        /*border: 1px solid red;*/
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
    .create-vault > h3 {
        background-color: #482b4861;
        border-radius: 6px;
        padding: 0.2rem 1rem;
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
        margin-bottom: 0.5rem;
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
    .plus.cross {
        transform: rotate(45deg);
        scale: 1.07;
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
</style>
