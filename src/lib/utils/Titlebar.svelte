<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";

    const appWindow = getCurrentWindow();

    const minimize = () => appWindow.minimize();
    const close = () => appWindow.close();
    // import { appWindow } from "@tauri-apps/api/window";
</script>

{#if appWindow}
    <div data-tauri-drag-region={true} class="titlebar">
        <div class="window-controls">
            <button onclick={minimize}>–</button>

            <button onclick={close}>×</button>
        </div>
    </div>
{/if}

<style>
    .titlebar {
        height: 32px;
        background: #121212;
        color: #eee;
        display: flex;
        justify-content: flex-end;
        align-items: center;
        -webkit-app-region: drag;

        /*border: 1px solid red;*/
    }
    .window-controls {
        display: flex;
        /*border: 1px solid red;*/
        margin-right: 0.4rem;
        gap: 0.4rem;
    }
    .window-controls button {
        -webkit-app-region: no-drag;
        background: transparent;
        border: none;
        color: #eee;
        padding: 0 12px;
        font-size: 16px;
        cursor: pointer;
    }

    .window-controls button:hover {
        background-color: #907da05c;
    }
</style>
