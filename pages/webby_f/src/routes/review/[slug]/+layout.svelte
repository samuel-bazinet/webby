<script>
	let { data, children } = $props();
</script>

<div class="layout">
	<main>
		{@render children()}
	</main>
</div>

<aside>
	<h2>More reviews</h2>
	<ul>
		{#each data.summaries as {slug, title}}
			<li>
				<a href="/review/{slug}" target="_self">{title}</a>
			</li>
		{/each}
	</ul>
</aside>

<style>
	@media (min-width: 640px) {
		.layout {
			display: grid;
			gap: 2em;
			grid-template-columns: 1fr 16em;
		}
	}
</style>
