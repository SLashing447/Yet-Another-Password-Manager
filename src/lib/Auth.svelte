<script lang="ts">
    import Operator from "../scripts/API";
    import Form from "./utils/Form.svelte";
    async function onUnlockDb({ password }: { password: string }) {
        await Operator.unlock("db", password);
    }
</script>

<div class="wrapper">
    <div class="texts">
        <span class="quote"> Yet Another <span>PASSWORD</span> Manager</span>
    </div>
    <div class="body">
        <div class="card">
            <h3>Login</h3>
            <Form
                inputs={[
                    {
                        name: "password",
                        placeholder: "Master Key",
                        required: true,
                        type: "pass",
                    },
                ]}
                SubmitBtn="Unlock"
                onSubmit={onUnlockDb}
            />
        </div>
    </div>
    <div class="texts">
        <span class="quote"> Powered By <span>RUST</span> Backend</span>
        <span class="quote link"> FOSS Application</span>
    </div>
</div>

<style>
    .wrapper {
        /*height: 100%;*/
        width: 100%;
        display: flex;
        /*justify-content: center;*/
        align-items: center;
        flex-direction: column;
        gap: 1rem;
    }
    .body {
        flex: 1;
        /*border: 1px solid white;*/
        display: flex;
        align-items: center;
        justify-content: center;
    }
    .texts {
        display: flex;
        flex-direction: column;
        text-align: center;
        gap: 0.5rem;
    }
    .quote {
        font-family: monospace;
        /*font-weight: bold;*/
        font-variant: small-caps;

        color: #d4cdcd;
    }
    .quote > span {
        font-style: italic;
        font-weight: bold;
        color: #a344f1;
    }
    .quote.link {
        cursor: pointer;
    }
    .quote.link:hover {
        text-decoration: underline;
    }
    .card {
        display: flex;
        flex-direction: column;
        padding: 1rem;
        gap: 0.6rem;
        background-color: #512f5136;
        border-radius: 5px;
        /*width: 20rem;*/

        /*border: 1px solid white;*/
    }
    .card > h3 {
        padding: 0.2rem 1rem;

        background-color: #482b4861;
    }
</style>
