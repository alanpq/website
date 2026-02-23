<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils';
	let {
		level,
		children,
		class: className,
		...props
	}: { level: 1 | 2 | 3 | 4 | 5 | 6 } & HTMLAttributes<HTMLHeadingElement> = $props();
</script>

<svelte:element this={`h${level}`} class={cn('group relative', className)} {...props}>
	{#if props.id}
		<a
			href="#{props.id}"
			class="absolute right-full box-border px-4 font-mono opacity-0 transition-opacity group-hover:opacity-60 hover:opacity-100 max-[700px]:hidden"
		>
			#
		</a>
	{/if}
	{@render children?.()}
</svelte:element>
