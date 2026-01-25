<script context="module">
  import Link from '$lib/components/ui/link.svelte';
  export { Link as a };
</script>

<script lang="ts">
  let {title, date}: {title: string, date: string} = $props();
</script>

<header class="prose box-content px-4 mx-auto max-w-[83ch] mt-10">
    <h1 class="text-6xl">{ title }</h1>
    <!-- <p class="date">on: { date }</p> -->
</header>
<main class="prose box-content px-4 mx-auto max-w-[80ch] mt-4 mb-15">
    <slot>
      <!-- the mdsvex content will be slotted in here -->
    </slot>
</main>
