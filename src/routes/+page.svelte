<script lang="ts">
	import { goto } from '$app/navigation';
	import { beforeUpdate } from 'svelte';

	beforeUpdate(() => {
		goto('/user/login');
	});
</script>
