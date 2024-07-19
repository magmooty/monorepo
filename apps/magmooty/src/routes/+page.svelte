<script>
	import { Button } from 'flowbite-svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	async function handleClick() {
		const response = await invoke('test_db');
		console.log(response);
	}
</script>

<Button on:click={handleClick}>Click me</Button>
