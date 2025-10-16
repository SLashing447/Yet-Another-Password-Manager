<script lang="ts">
    interface props {
        name: string;
        dsc: string;
        img: string;
        selected?: boolean;
        onClick?: () => void;
    }

    let props: props = $props();
    let { dsc, img, name } = props;

    // $effect(() => console.log("selected: "));

    function onClick() {
        // console.log("here");
        if (props.onClick) props.onClick();
    }
</script>

<button class={`wrapper ${props.selected ? "sel" : ""}`} onclick={onClick}>
    <div class="img">
        <img src={img} alt={name} />
    </div>
    <div class="data">
        <span class="name">{name}</span>
        <span class="dsc">{dsc.slice(0, 15 - 1) + "…"}</span>
    </div>
</button>

<style>
    .wrapper {
        /* display: inline-block; */
        display: flex;
        /* border: 2px solid var(--acc1); */
        /* width:; */
        padding: 0.2rem 0.8rem;
        gap: 0.5rem;

        cursor: pointer;
        user-select: none;
        border-radius: 8px;
        background-color: transparent;
        outline: none !important;
        border: none;
        transition: 0.2 all ease;
        height: fit-content;

        width: 100%;
        /* max-width: 200px; */
    }
    .wrapper.sel {
        box-shadow: inset 0 0 0 2px var(--acc2);
        background-color: rgba(54, 49, 90, 0.65);
    }
    .wrapper:hover {
        background-color: rgba(178, 139, 246, 0.144);
    }
    .img {
        width: 30px;
        display: flex;
    }
    .name {
        font-weight: bold;
    }
    .dsc {
        color: rgb(176, 170, 170);
        font-size: 12px;
        /* white-space: nowrap; */
    }
    .img > img {
        width: 100%;
    }
    .data {
        display: flex;
        flex-direction: column;
        align-items: start;
    }
</style>
