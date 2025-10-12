<script lang="ts">
  import { checkPasswordStrength } from "../../scripts/password";

  interface props {
    type: number;
    onPass?: (pass: string) => void;
    width?: string;
    placeholder?: string;
  }
  let props: props = $props();
  let type = props.type;

  let passSt: {
    color: string;
    width: string;
  } | null = $state(null);

  let focus: boolean = $state(false);
  let vis: boolean = $state(false);
  let pass: string = $state("");

  function onKeyDown(e: KeyboardEvent) {
    const key = e.key;
    if (key === "Enter" && pass.trim() !== "" && props.onPass)
      props.onPass(pass);
  }

  $effect(() => {
    if (props.type === 2) {
      let st = checkPasswordStrength(pass);
      passSt = {
        color: st.color,
        width: `${(st.score / 5) * 100}%`,
      };
    }
  });
</script>

<div class="wrapper" style={props.width ? `width:${props.width}` : ""}>
  <div
    class={`box ${focus ? "foc" : ""}`}
    onfocus={() => (focus = true)}
    tabindex={1}
    role="button"
  >
    <input
      bind:value={pass}
      onkeydown={onKeyDown}
      onblur={() => (focus = false)}
      onfocus={() => (focus = true)}
      type={vis ? "text" : "password"}
      placeholder={props.placeholder ? props.placeholder : "Password"}
    />
    <button class="vis" onclick={() => (vis = !vis)}>{vis ? "👁️" : "🙈"}</button
    >
  </div>
  {#if type === 1}
    <button class="rst"> Reset Password </button>
  {/if}
  {#if type === 2 && passSt}
    <div class="st-bar">
      <span style={`background-color: ${passSt.color};width:${passSt.width}`}
      ></span>
    </div>
  {/if}
</div>

<style>
  .wrapper {
    display: flex;
    gap: 0.2rem;
    flex-direction: column;
    /* border: 1px solid red; */
  }
  .rst {
    background-color: transparent !important;
    text-align: left;
    font-family: monospace;
    font-size: 12px;
    cursor: pointer;
    padding: 0 0.3rem;
  }
  .rst:hover {
    text-decoration: underline;
  }

  .st-bar {
    width: 100%;
    padding: 0 0.2rem;
    border-radius: 3px;
    display: flex;
    /* border: 1px solid red; */
    background-color: transparent;
  }
  .st-bar > span {
    width: 50%;
    transition: 0.2s all ease;
    border-radius: 2px;

    padding: 0.2rem 0;
    background-color: red;
    /* height: 100%; */
  }
  .vis {
    padding: 0.3rem 0.6rem;
    border-radius: 4px;
    background-color: transparent;
    font-size: 17px;
    cursor: pointer;
  }
  .vis:hover {
    background-color: rgba(171, 171, 171, 0.276);
  }
  input::placeholder {
    color: rgb(203, 201, 201);
  }
  input,
  button {
    outline: none !important;
    border: none !important;
    font-size: 16px;
  }
  input {
    background-color: transparent !important;
    /* line-height: 20px; */
    width: 100%;
  }
  .box {
    display: flex;
    gap: 0.3rem;
    width: 100%;
    /* border: 1px solid grey; */
    box-shadow: inset 0 0 0 1px var(--acc1);
    border-radius: 4px;
    background-color: rgba(51, 50, 50, 0.45);
    padding: 0.3rem 1rem;
  }
  .box.foc > input::placeholder {
    font-weight: 600;
  }
  .box.foc {
    box-shadow: inset 0 0 0 2px var(--acc1);
  }
</style>
