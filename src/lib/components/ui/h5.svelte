<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import Heading from './heading.svelte';
	let { children, ...props }: HTMLAttributes<HTMLHeadingElement> = $props();
</script>

<Heading level={5} {...props}>
	{@render children?.()}
</Heading>
