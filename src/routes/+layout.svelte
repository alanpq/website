<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<nav class="flex gap-4 px-20 pt-15">
	<a>Alan P</a>
	<span class="grow"></span>
	<a href="/">Home</a>
	<a href="/posts">Posts</a>
</nav>

{@render children()}
