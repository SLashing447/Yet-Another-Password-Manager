<script lang="ts">
  import { routeBack, routes } from "../scripts/utils";

  let toptTimerWidth: number = $state(100);

  setInterval(() => {
    toptTimerWidth = toptTimerWidth - 1;
    if (toptTimerWidth === 0) toptTimerWidth = 100;
  }, 300);

  function onClose() {
    routeBack();
  }
</script>

<div class="wrapper">
  <div class="body">
    <div class="box">
      <h3>Steam</h3>
      <div class="details">
        <span> Username: BLack_Pigeon7 </span>
        <span> email: aishikchakraborty006@gmail.com</span>
        <span class="pass">Copy Password</span>
      </div>
    </div>
    <div class="box">
      <span style={`width:${toptTimerWidth}%`} class="timer"></span>
      <h3>TOPT (2FA)</h3>
      <div class="code">
        <span>3</span>
        <span>4</span>
        <span>1</span>
        <span>2</span>
        <span>7</span>
        <span>3</span>
      </div>
    </div>
  </div>
  <div class="pannel">
    <button onclick={onClose}>{"❌"} </button>
    <button> {"🗑️"} </button>
    <button> {"🔢"} </button>

    <button> {"🖊️"} </button>
  </div>
</div>

<style>
  .wrapper {
    /* border: 1px solid red; */
    display: flex;

    gap: 0.2rem;

    justify-content: center;
    margin: 1rem;
  }
  .pannel > button {
    outline: none !important;
    border: none !important;
    border-radius: 3px;
    width: 35px;
    height: 35px;
    cursor: pointer;
    /* padding: 0.6rem 1rem; */
    background-color: rgba(140, 101, 208, 0.209);
  }
  .pannel > button:hover {
    background-color: rgba(184, 151, 242, 0.209);
  }
  .pass {
    font-variant: small-caps;
    /* font-weight: bold; */
    font-size: small;
    cursor: pointer;
    user-select: none;
    width: fit-content;
  }
  .pass:hover {
    text-decoration: underline;
  }
  .pass:active {
    scale: 0.98;
  }
  .pannel {
    display: flex;
    gap: 0.2rem;
    flex-direction: column;
  }
  .body {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 0.2rem;
    width: 70%;
    /* border: 1px solid red; */
  }
  .details {
    display: flex;
    flex-direction: column;
  }
  .details > span {
    font-size: 15px;
  }
  .timer {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    border-radius: 5px;
    background-color: rgba(122, 82, 167, 0.209);
    z-index: 1;
    transition: 1s all ease;
  }
  .box {
    position: relative;
    width: auto;
    flex-grow: 1;
    background-color: rgba(140, 101, 208, 0.209);
    border-radius: 5px;
    padding: 0.3rem 0.8rem;
  }
  .code > span {
    font-family: monospace;
    font-weight: bold;
    font-size: 16px;
    user-select: none;
  }
  .code {
    display: flex;
    gap: 0.3rem;
    justify-content: center;
  }
</style>
