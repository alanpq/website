<script module>
	import Link from '$lib/components/ui/link.svelte';
	import Code from '$lib/components/ui/code.svelte';
	import Pre from '$lib/components/ui/pre.svelte';
	import type { Snippet } from 'svelte';
	export { default as h1 } from '$lib/components/ui/h1.svelte';
	export { default as h2 } from '$lib/components/ui/h2.svelte';
	export { default as h3 } from '$lib/components/ui/h3.svelte';
	export { default as h4 } from '$lib/components/ui/h4.svelte';
	export { default as h5 } from '$lib/components/ui/h5.svelte';
	export { default as h6 } from '$lib/components/ui/h6.svelte';
	export { default as img } from '$lib/components/ui/img.svelte';
	export { Link as a, Pre as pre };
</script>

<script lang="ts">
	let { children }: { children?: Snippet<[]> } = $props();
</script>

{@render children?.()}
