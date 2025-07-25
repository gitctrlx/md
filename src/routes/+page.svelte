<script lang="ts">
  import { onMount, tick } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import katex from "katex";
  import hljs from "highlight.js";

  import "./app.scss";
  import defaultMd from "./example.md?raw";

  let html = "";

  onMount(async () => {
    html = await invoke("md_to_html", {
      md: defaultMd,
    });
    await tick();

    const maths = document.querySelectorAll("[data-math-style]");
    maths.forEach((e) => {
      katex.render(e.textContent!, e as HTMLElement);
    });

    hljs.highlightAll();
  });
</script>

<main>
  {@html html}
</main>

<style>
  main {
    padding: 2rem;
    font-family: sans-serif;
    line-height: 1.6;
  }
</style>
