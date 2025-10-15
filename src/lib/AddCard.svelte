<script lang="ts">
    import Operator, { DataType } from "../scripts/API";
    import type { CardSchema } from "../scripts/types";
    // import Card from "./Card.svelte";
    import Router from "./Router.svelte";
    let formEl: HTMLFormElement;

    let msg: string = $state("");
    let cnfmPass: string = $state("");

    async function onSubmit(e: SubmitEvent) {
        e.preventDefault();
        msg = "";
        const data = new FormData(formEl);
        const obj: CardSchema = Object.fromEntries(data.entries());
        if (obj.password !== cnfmPass) {
            msg = "Password & Cofirm Password doesnot Match";
            return;
        }

        await Operator.post(DataType.card, obj);
    }
</script>

<div class="wrapper">
    <h2>Vault XYZ</h2>
    <Router />
    <span>Add Connection </span>
    <form bind:this={formEl} onsubmit={onSubmit} class="form">
        <div class="input">
            <span> 📎 </span>
            <input
                required
                name="provider"
                type="text"
                placeholder="Provider Name"
            />
        </div>
        <div class="input">
            <span> 👤 </span>
            <input
                type="text"
                name="username"
                placeholder="Username (Optional)"
            />
        </div>
        <div class="input">
            <span>✉️</span>
            <input type="text" name="email" placeholder="e-mail (Optional)" />
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
                bind:value={cnfmPass}
                required
                type="password"
                placeholder="Confirm Password"
            />
        </div>
        <div class="input btn">
            <button>Add</button>
        </div>
        {msg}
    </form>
</div>

<style>
    .wrapper {
        flex: 1;
        padding: 0.5rem 0.5rem 0 0.5rem;
        display: flex;

        flex-direction: column;
    }
    input,
    button {
        outline: none !important;
        border: none !important;
        background-color: transparent !important;
    }

    .wrapper > span {
        background-color: rgba(144, 58, 144, 0.29);
        width: fit-content;
        padding: 0.2rem 1rem;
        margin-top: 0.5rem;
    }
    .input > button {
        cursor: pointer;
    }
    .input > button:hover {
        background-color: red;
    }
    .input > button,
    input {
        width: 100%;
    }
    .input.btn:hover {
        background-color: rgba(76, 22, 111, 0.175);
    }
    .input > span {
        font-size: 13px;
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
        margin: 2rem 0;
        align-self: center;
        background-color: rgba(87, 70, 113, 0.217);
        padding: 0.5rem;
        border-radius: 5px;
    }
</style>
