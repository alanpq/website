<script module>
  import Link from '$lib/components/ui/link.svelte';
  import Code from '$lib/components/ui/code.svelte';
  export { Link as a, Code as code, Code as pre };
</script>

<script lang="ts">
  let {title, date}: {title: string, date: string} = $props();
</script>

    <slot>
      <!-- the mdsvex content will be slotted in here -->
    </slot>
