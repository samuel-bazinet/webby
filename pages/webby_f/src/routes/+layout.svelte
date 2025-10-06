<script lang="ts">
	import favicon from "$lib/assets/favicon.svg";
	import logo from "$lib/assets/rotisserie-kitten.svg"

	let { children } = $props();

	function toggleMenu() {
		var x = document.getElementById("links");
		if (x!.style.display === "block") {
			x!.style.display = "none";
		} else {
			x!.style.display = "block";
		}
	}
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Rotisserie Kittens</title>
</svelte:head>

<div class="container">
	<div class="bar">

		<div class="logo">
			<img src={logo} alt="Rotisserie Kittens">
		</div>
		
		<div class="sidenav" id="links">
			<ul style="all: unset;">
				<li><a href="/" target="_self">Home</a></li>
				<li><a href="/review" target="_self">Reviews</a></li>
			</ul>
		</div>
	</div>
	
	<div class="main">
		{@render children?.()}
	</div>
</div>
	
<style>
	:global(body) {
		margin: 0;
		padding: 0;
	}

	.container {
		top: 0;
		left: 0;
		height: 100%;
		width: 100%;
		display: grid;
		grid-template-areas: 
			"bar content";
		grid-template-columns: 7% 93%;
		margin: 0;
		padding: 0;
	}

	.bar {
		grid-area: bar;
		display: grid;
		grid-template-areas: 
		"logo"
		"nav";
		grid-template-rows: 12.4% 87.6%;
		position: fixed;
		height: 100%;
	}

	.logo {
		background-color: rgb(137, 44, 0);
		background-size: contain;
		grid-area: logo;
		padding: 0.1vw;
	}

	.sidenav {
		background-color: #ffd500;
		padding-top: 20px;
		grid-area: nav;
	}

	.sidenav a {
		padding: 0.1vw 0.5vw 0.1vw 0.5vw;
		text-decoration: none;
		font-size: 1.5vw;
		color: #4f4f4f;
		display: block;
	}

	.sidenav a:hover {
		color: #f1f1f1;
	}

	.main {
		padding: 1%;
		grid-area: content;
	}
</style>
