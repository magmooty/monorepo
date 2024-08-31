<script>
	import AcademicYearSelector from 'components/AcademicYearSelector.svelte';
	import Button from 'components/common/button/Button.svelte';
	import { _ } from 'svelte-i18n';
</script>

<div class="flex h-screen w-screen items-center justify-center">
	<div class="w-max-[300px] flex flex-col gap-5">
		<h1 class="w-full text-right text-3xl">{$_('onboarding.academicYearQuestion')}</h1>
		<AcademicYearSelector />
		<Button class="w-full">{$_('common.continue')}</Button>
	</div>
</div>
