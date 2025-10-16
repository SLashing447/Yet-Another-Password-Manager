<script lang="ts">
    import Operator, { DataType } from "../scripts/API";
    import type { CardSchema, FormProps } from "../scripts/types";
    import Form from "./utils/Form.svelte";
    // import Card from "./Card.svelte";
    import Router from "./Router.svelte";

    async function onSubmit(e: CardSchema['on_create']) {
        await Operator.addCard(e);
    }

    const INPUTS: FormProps[] = [
        {
            name: "📎/provider",
            placeholder: "Provider Name",
            required: true,
        },
        {
            name: "👤/username",
            placeholder: "Username (Optional)",
        },
        {
            name: "✉️/email",
            placeholder: "Email (Optional)",
        },
        {
            name: "📝/dsc",
            placeholder: "Description (Optional)",
            required: true,
        },
        {
            name: "🔑/password",
            placeholder: "Password",
            required: true,
            type: "pass",
            stMeter: true,
        },
        {
            name: "🔑/Confirm Password",
            type: "cnfm-pass",
        },
    ];
</script>

<div class="wrapper">
    <h2>Vault XYZ</h2>
    <Router />
    <span>Add Connection </span>
    <div class="form">
        <div>
            <Form inputs={INPUTS} {onSubmit} SubmitBtn="Add" />
        </div>
    </div>
</div>

<style>
    .form {
        width: 100%;
        display: flex;
        justify-content: center;
        margin-top: 1rem;
    }
    .form > div {
        width: 70%;
    }
    .wrapper {
        flex: 1;
        padding: 0.5rem 0.5rem 0 0.5rem;
        display: flex;

        flex-direction: column;
    }

    .wrapper > span {
        background-color: rgba(144, 58, 144, 0.29);
        width: fit-content;
        padding: 0.2rem 1rem;
        margin-top: 0.5rem;
    }
</style>
