<script lang="ts">
	import type { HTMLPreAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils';

	let { class: className, ...props }: HTMLPreAttributes = $props();
</script>

<pre class={cn('', className)} {...props}>{@render props.children?.()}</pre>
