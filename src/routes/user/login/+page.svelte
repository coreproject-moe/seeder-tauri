<script lang="ts">
	import LoginFormSkeleton from '$skeletons/login_page.svelte';
	import type { ComponentType } from 'svelte';

	// We import our page components (similar to the one above).
	let one = import('./1.svelte');
	let two = import('./2.svelte');

	const pages = [one, two];

	// The current page of our form.
	let page = 0;

	// The state of all of our pages
	let pages_state: { [key: number]: Record<string, string | number> } = [];

	// Our handlers
	function onSubmit(values: CustomEvent) {
		if (page === pages.length - 1) {
			// end of the page. do something.. Maybe seek for anime girls with stockings and leggings
			// XD
		} else {
			// If we're not on the last page, store our data and increase a step
			pages_state[page] = values.detail;
			page += 1;
		}
	}

	// Do some wizardy here later
	// https://github.com/sveltejs/language-tools/issues/486#issuecomment-1372330560
	let current_page: Promise<{ default: ComponentType }>;

	$: current_page = pages[page];
</script>

<!-- We display the current step here -->
{#await current_page}
	<LoginFormSkeleton />
{:then Module}
	<svelte:component this={Module.default} on:submit={onSubmit} />
{/await}
