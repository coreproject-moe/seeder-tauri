import type { Writable } from 'svelte/store';
import { localStorageStore } from '@skeletonlabs/skeleton';

export const is_authenticated: Writable<boolean> = localStorageStore('authenticated', false);
