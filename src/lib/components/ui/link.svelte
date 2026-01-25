<script lang="ts">
	import type { HTMLAnchorAttributes } from 'svelte/elements';
    import {cn} from '$lib/utils';

    let {class: className, ...props}: HTMLAnchorAttributes = $props();
</script>

<a class={cn("underline-offset-3 hover:bg-primary hover:text-background hover:shadow-[0_0_0_3px_var(--color-primary)]", className)} {...props}>{@render props.children?.()}</a>
