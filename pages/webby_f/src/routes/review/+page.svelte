<script lang="ts">
	let {data} = $props();
</script>

<h1>Reviews</h1>

<ul>
	{#each data.summaries as {slug, title}}
		<li><a href="/review/{slug}">{title}</a></li>
	{/each}
</ul>
