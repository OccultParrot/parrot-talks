<script lang="ts">
	import './layout.css';
	import favicon from '#lib/assets/favicon.svg';
	import type { LayoutProps } from './$types';

	let { children }: LayoutProps = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="min-h-screen bg-white text-neutral-800 antialiased">
	<header
		class="sticky top-0 z-10 border-t-4 border-b border-green-600 border-b-green-100 bg-white/80 backdrop-blur"
	>
		<div class="mx-auto flex max-w-2xl items-center gap-2 px-6 py-4">
			<!-- <span class="h-3 w-3 rounded-full bg-green-500"></span> -->
			<a
				href="/"
				class="text-xl font-extrabold tracking-tight text-green-700 transition-colors hover:text-green-600"
			>
				Parrot Yaps
			</a>
		</div>
	</header>

	<main class="mx-auto max-w-2xl px-6 py-12">
		{@render children()}
	</main>
</div>
