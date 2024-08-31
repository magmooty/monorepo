<script lang="ts">
	import { Label, Select } from 'flowbite-svelte';
	import { _ } from 'svelte-i18n';
	let value: string;

	let countries = [
		{ value: '2023', name: '2023' },
		{ value: '2024', name: '2024' },
		{ value: '2025', name: '2025' }
	];
</script>

<Label class="w-[300px]">
	{$_('onboarding.academicYear')}
	<Select class="mt-2" items={countries} bind:value on:change placeholder={$_('common.choose')} />
</Label>
