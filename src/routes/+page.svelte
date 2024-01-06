<script lang="ts">
	import { goto } from '$app/navigation';
	import { beforeUpdate } from 'svelte';
	import { is_authenticated } from '$store/user';
	import { get } from 'svelte/store';
	beforeUpdate(() => {
		if (get(is_authenticated) === false) {
			goto('/user/login');
		}
	});
</script>
