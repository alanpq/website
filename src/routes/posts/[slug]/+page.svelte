<script lang="ts">
	import { formatDate } from '$lib/utils';

	let { data } = $props();
</script>

<svelte:head>
	<title>{data.meta.title}</title>
	<meta property="og:type" content="article" />
	<meta property="og:title" content={data.meta.title} />
</svelte:head>

<main class="layout-container prose">
	<header class="not-prose popout mt-10 mb-5 box-content flex flex-col gap-2 font-black">
		<hgroup>
			<h1 class="text-6xl">{data.meta.title}</h1>
		</hgroup>

		<div class="flex gap-2">
			<p class="text-muted-foreground">{formatDate(data.meta.date)}</p>
			<span class="grow"></span>
			{#each data.meta.categories as category}
				<span class="text-muted-foreground/50 italic">&num;{category}</span>
			{/each}
		</div>
	</header>

	<data.content />
</main>
