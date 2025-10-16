<div class="wrapper">
    <button class="sel">
        <img src="src/assets/hdd.png" alt="hello" />
    </button>
    <button>
        <img src="src/assets/gd.png" alt="hello" />
    </button>
    <button>
        <img src="src/assets/od.png" alt="hello" />
    </button>
</div>

<style>
    .wrapper {
        /*border: 1px solid red;*/
        height: fit-content;
        padding: 0.1rem;
        display: flex;
        /*justify-content: center;*/
        gap: 0.2rem;
    }
    button:hover {
        background-color: #6438748a;
    }
    button {
        background-color: transparent;
        outline: none !important;
        border: 2px solid transparent;
        border-radius: 3px;
        display: flex;
        padding: 0.3rem 0.3rem;
        cursor: pointer;
    }
    button.sel {
        background-color: #6438748a;
        border-color: #74559a;
    }
    img {
        width: 20px;
    }
</style>
