<script module>
	import Link from '$lib/components/ui/link.svelte';
	import Code from '$lib/components/ui/code.svelte';
	import type { Snippet } from 'svelte';
	export { Link as a };
</script>

<script lang="ts">
	let { children }: { children?: Snippet<[]> } = $props();
</script>

{@render children?.()}
