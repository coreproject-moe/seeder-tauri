<script lang="ts">
	import Login from './login.svelte';
	import UploadPage from './upload_page.svelte';

	let token: string | null = null; // check if token is valid

	function on_submit(values: CustomEvent) {
		token = values.detail;
	}
</script>

{#if token}
	<UploadPage />
{:else}
	<!-- logic for testing -->
	<Login on:submit={on_submit} />
{/if}
