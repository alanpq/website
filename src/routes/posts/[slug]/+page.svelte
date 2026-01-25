<script lang="ts">
	import { formatDate } from '$lib/utils'

	let { data } = $props()
</script>

<svelte:head>
	<title>{data.meta.title}</title>
	<meta property="og:type" content="article" />
	<meta property="og:title" content={data.meta.title} />
</svelte:head>

<article>
    <header class="font-black box-content px-4 mx-auto max-w-[83ch] mt-10">
        <hgroup>
            <h1 class="text-6xl">{data.meta.title}</h1>
            <p>Published at {formatDate(data.meta.date)}</p>
        </hgroup>

        <div class="tags">
            {#each data.meta.categories as category}
                <span class="surface-4">&num;{category}</span>
            {/each}
        </div>
    </header>

	<main class="prose box-content px-4 mx-auto max-w-[80ch] mt-4 mb-15">
		<data.content />
	</main>
</article>

