import { mount } from "svelte";
import App from "./App.svelte";
import "./styles/global.css";
import "./styles/themes.css";
// import { invoke } from "@tauri-apps/api/core";

// let summary = await invoke("note_command");
// console.log("Summary : ", summary);

const app = mount(App, {
  target: document.getElementById("app")!,
});

export default app;
