<script lang="ts">
    import type { FormProps } from "../../scripts/types";
    import {
        evaluatePassword,
        type PasswordStrength,
    } from "../../scripts/utils";

    interface props {
        inputs: FormProps[]; // name of the input fields
        onSubmit?: (obj: any) => void;
        SubmitBtn?: string;
    }

    let props: props = $props();
    let { inputs, SubmitBtn = "Submit" } = props;

    let formEl: HTMLFormElement;
    let cnfmPassTxt: string = $state("");
    let passTxt: string = $state("");

    let isPassVis: boolean = $state(false);
    let passEval: PasswordStrength | null = $state(null);

    let error: string = $state("");

    $effect(() => {
        passEval = evaluatePassword(passTxt);
    });

    function onSubmit(e: SubmitEvent) {
        e.preventDefault();
        if (error) error = "";

        const data = new FormData(formEl);
        const obj = Object.fromEntries(data.entries());

        if (passTxt.trim() !== "") {
            if (passTxt !== cnfmPassTxt) {
                error = "Passwords Do not match";
                return;
            }
        }
        if (props.onSubmit) props.onSubmit(obj);
    }

    function getType(type: string) {
        if (type === "cnfm-pass" || type === "pass") return "password";
        return "text";
    }

    function getPlaceholder(placeholder: string) {
        return placeholder.includes("/")
            ? placeholder.split("/")[1]
            : placeholder;
    }
</script>

<div class="wrapper">
    <form bind:this={formEl} onsubmit={onSubmit}>
        {#each inputs as { name, type = 'text', placeholder = name, required = false, stMeter = false }, k}
            <div class="input">
                {#if name.includes("/") && name.split("/")[0]}
                    <span class="icon">{name.split("/")[0]}</span>
                {/if}
                {#if type === "cnfm-pass"}
                    <input
                        bind:value={cnfmPassTxt}
                        type="password"
                        {required}
                        placeholder={getPlaceholder(placeholder)}
                    />
                {:else if type === "pass"}
                    <input
                        {required}
                        bind:value={passTxt}
                        type={isPassVis ? "text" : "password"}
                        name={name.split("/")[1] || name}
                        placeholder={getPlaceholder(placeholder)}
                    />
                {:else}
                    <input
                        {required}
                        type={getType(type)}
                        name={name.split("/")[1] || name}
                        placeholder={getPlaceholder(placeholder)}
                    />
                {/if}

                {#if type === "pass" && !name.split("/")[2]}
                    <button
                        onclick={() => (isPassVis = !isPassVis)}
                        type="button"
                        class="visp"
                    >
                        {!isPassVis ? "🙈" : "🐵"}
                    </button>
                {:else if name.includes("/") && name.split("/")[2]}
                    <span class="icon">{name.split("/")[2]}</span>
                {/if}
            </div>
            {#if type === "pass" && stMeter && passEval && passEval.score !== 0}
                <div class="st-meter">
                    <span
                        style={`width:${passEval.score * 10}%;background-color:${passEval.color}`}
                    ></span>
                </div>
            {/if}
        {/each}
        {#if SubmitBtn}
            <button type="submit" class="submit"> {SubmitBtn} </button>
        {/if}
    </form>
    {#if error}
        <div class="error">{error}</div>
    {/if}
</div>

<style>
    input,
    button {
        outline: none !important;
        background-color: transparent;
    }
    .error {
        /*border: 1px solid grey;*/
        font-size: 12px;
        color: #cb4848;
        padding: 0.1rem 0.4rem;
        font-weight: bold;
        font-family: monospace;
        text-transform: capitalize;
    }
    input {
        border: none !important;
        flex: 1;
    }
    .icon {
        font-size: 14px;
        user-select: none;
        cursor: default;
    }

    input::placeholder {
        /*font-weight: bold;*/
        color: lightgrey;
    }
    .st-meter {
        display: flex;
    }
    .st-meter > span {
        border-radius: 3px;
        padding: 0.16rem;
        transition: 0.15s all ease;
    }

    form {
        /*background-color: ;*/
        display: flex;
        flex-direction: column;
        gap: 0.4rem;
        padding: 0.2rem;
    }
    .input:focus {
        border-width: 3px;
    }
    input:focus::placeholder {
        font-weight: bold;
    }

    .visp {
        border: transparent;
        padding: 0 0.2rem;
        border-radius: 3px;
        font-size: 14px;

        cursor: pointer;
    }
    .visp:hover {
        background-color: #59466754;
    }
    .submit:hover {
        background-color: #59466754;
    }
    .submit {
        border: 3px solid var(--acc2);
        width: 100%;
        border-radius: 3px;
        cursor: pointer;
        font-weight: bold;
        padding: 0.2rem 0;
    }
    .input {
        border: 3px solid var(--acc2);
        background-color: #694d693d;
        border-radius: 5px;
        width: 100%;
        padding: 0.19rem 1rem;
        display: flex;
        gap: 0.3rem;
    }
</style>
