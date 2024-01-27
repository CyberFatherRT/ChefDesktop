import "./styles.css";
import App from "./App.svelte";

export const prerender = true
export const ssr = false

const app = new App({
    target: document.getElementById("app")
});

export default app;
