<script>
	import { createForm } from 'svelte-forms-lib';
	import * as yup from 'yup';
	import { Label, Input } from 'flowbite-svelte';
	import Button from './common/button/Button.svelte';
	import { Alert } from 'flowbite-svelte';
	import { _ } from 'svelte-i18n';

	const { form, errors, state, handleChange, handleSubmit } = createForm({
		initialValues: {
			name: '',
			password: ''
		},
		validationSchema: yup.object().shape({
			name: yup.string().required($_('onboarding.nameIsRequired')),
			password: yup
				.string()
				.min(8, $_('onboarding.passwordMinLength'))
				.required($_('onboarding.passwordIsRequired'))
		}),
		onSubmit: (values) => {
			console.log(JSON.stringify(values));
		}
	});
</script>

<form on:submit={handleSubmit}>
	<div class="mb-6">
		<Label for="name" class="mb-2 block">{$_('common.name')}</Label>
		<Input
			id="name"
			name="name"
			on:change={handleChange}
			bind:value={$form.name}
			on:blur={handleChange}
			placeholder={$_('common.yourName')}
			type="text"
		/>
		{#if $errors.name}
			<Alert color="red" class="mt-4">
				{$errors.name}
			</Alert>
		{/if}
	</div>
	<div class="mb-6">
		<Label for="password" class="mb-2 block">{$_('common.password')}</Label>
		<Input
			id="password"
			name="password"
			on:change={handleChange}
			on:blur={handleChange}
			bind:value={$form.password}
			placeholder={$_('common.password')}
			type="password"
		/>
		{#if $errors.password}
			<Alert color="red" class="mt-4">
				{$errors.password}
			</Alert>
		{/if}
	</div>
	<Button type="submit" class="w-full">{$_('common.continue')}</Button>
</form>
