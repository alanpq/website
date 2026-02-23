<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils';
	let { class: className, ...props }: HTMLAttributes<HTMLImageElement> = $props();
</script>

<img class={cn(className)} loading="lazy" {...props} />
