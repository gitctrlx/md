<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import katex from 'katex';
  import hljs from 'highlight.js';

  import './app.scss';
  import defaultMd from './example.md?raw';

  // =============================================================================
  // Types
  // =============================================================================

  interface TocItem {
    id: string;
    text: string;
    level: number;
    element: HTMLElement;
  }

  // =============================================================================
  // State
  // =============================================================================

  let html = '';
  let tocVisible = false; // Default to hidden
  let activeHeading = '';
  let tocItems: TocItem[] = [];

  // =============================================================================
  // TOC Management
  // =============================================================================

  function toggleToc(): void {
    tocVisible = !tocVisible;
  }

  function generateToc(): void {
    const headers = document.querySelectorAll(
      'main h1, main h2, main h3, main h4, main h5, main h6'
    ) as NodeListOf<HTMLHeadingElement>;

    tocItems = Array.from(headers).map((header, index) => {
      if (!header.id) {
        const text = header.textContent || '';
        header.id = `heading-${index}-${text
          .toLowerCase()
          .replace(/[^\w\s-]/g, '')
          .replace(/\s+/g, '-')}`;
      }

      return {
        id: header.id,
        text: header.textContent || '',
        level: parseInt(header.tagName.charAt(1)),
        element: header
      };
    });
  }

  function scrollToHeading(id: string): void {
    const element = document.getElementById(id);
    if (element) {
      element.scrollIntoView({ behavior: 'smooth', block: 'start' });
      activeHeading = id;
    }
  }

  // =============================================================================
  // Scroll Handling
  // =============================================================================

  function handleScroll(): void {
    let currentHeading = '';

    for (let i = tocItems.length - 1; i >= 0; i--) {
      const item = tocItems[i];
      const rect = item.element.getBoundingClientRect();

      if (rect.top <= window.innerHeight * 0.3) {
        currentHeading = item.id;
        break;
      }
    }

    if (currentHeading !== activeHeading) {
      activeHeading = currentHeading;
    }
  }

  function scrollToTop(): void {
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }

  // =============================================================================
  // Content Processing
  // =============================================================================

  async function renderMath(): Promise<void> {
    const mathElements = document.querySelectorAll('[data-math-style]');
    
    mathElements.forEach((element) => {
      try {
        const isDisplayMode = element.getAttribute('data-math-style') === 'display';
        katex.render(element.textContent!, element as HTMLElement, {
          displayMode: isDisplayMode,
          throwOnError: false
        });
      } catch (error) {
        console.error('KaTeX rendering error:', error);
      }
    });
  }

  async function highlightCode(): Promise<void> {
    hljs.highlightAll();
  }

  // =============================================================================
  // Initialization
  // =============================================================================

  async function initializeApp(): Promise<void> {
    try {
      // Render markdown to HTML
      html = await invoke('md_to_html', { md: defaultMd });
      
      // Wait for DOM updates
      await tick();
      await tick();
      
      // Generate table of contents
      setTimeout(generateToc, 100);
      
      // Process math and code
      await renderMath();
      await highlightCode();
      
      // Setup scroll handling
      window.addEventListener('scroll', handleScroll, { passive: true });
      handleScroll();
    } catch (error) {
      console.error('App initialization error:', error);
    }
  }

  // =============================================================================
  // Lifecycle
  // =============================================================================

  onMount(() => {
    initializeApp();

    return () => {
      window.removeEventListener('scroll', handleScroll);
    };
  });
</script>

<!-- =============================================================================
     Table of Contents Sidebar
     ============================================================================= -->
<nav class="toc" class:visible={tocVisible} aria-label="Table of Contents">
  {#if tocVisible}
    <div class="toc-content">
      {#each tocItems as item (item.id)}
        <a
          href="#{item.id}"
          class="toc-link level-{item.level}"
          class:active={activeHeading === item.id}
          on:click|preventDefault={() => scrollToHeading(item.id)}
          aria-current={activeHeading === item.id ? 'page' : undefined}
        >
          {item.text}
        </a>
      {/each}
    </div>
  {/if}
</nav>

<!-- =============================================================================
     Action Buttons
     ============================================================================= -->
<div class="button-group">
  <!-- TOC Toggle Button -->
  <button
    class="toc-toggle-btn"
    on:click={toggleToc}
    aria-label={tocVisible ? 'Hide Table of Contents' : 'Show Table of Contents'}
  >
    {#if tocVisible}
      <!-- Hide TOC Icon -->
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
        <path
          d="M15 18L9 12L15 6"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    {:else}
      <!-- Show TOC Icon -->
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
        <path
          d="M8 6H21M8 12H21M8 18H21M3 6H3.01M3 12H3.01M3 18H3.01"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    {/if}
  </button>

  <!-- Back to Top Button -->
  <button
    class="back-to-top"
    on:click={scrollToTop}
    aria-label="Back to Top"
  >
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
      <path
        d="M7 14L12 9L17 14"
        stroke="currentColor"
        stroke-width="2.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
    </svg>
  </button>
</div>

<!-- =============================================================================
     Main Content
     ============================================================================= -->
<main>
  {@html html}
</main>
