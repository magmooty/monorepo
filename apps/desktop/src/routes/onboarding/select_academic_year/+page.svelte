<script>
	import AcademicYearSelector from 'components/AcademicYearSelector.svelte';
	import Button from 'components/common/button/Button.svelte';
	import { _ } from 'svelte-i18n';
</script>

<div class="flex h-screen w-screen flex-col items-center justify-center gap-4">
	<h1>{$_('onboarding.academicYearQuestion')}</h1>
	<AcademicYearSelector />
	<Button class="w-[300px]">{$_('common.continue')}</Button>
</div>
