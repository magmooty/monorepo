<script>
	import { createForm } from 'svelte-forms-lib';
	import { Label, Input } from 'flowbite-svelte';

	import * as yup from 'yup';
	import { _ } from 'svelte-i18n';
	import Button from 'components/common/button/Button.svelte';
	const { form, state, handleChange, handleSubmit, errors } = createForm({
		initialValues: {
			password: ''
		},
		validationSchema: yup.object().shape({
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
	<Label for="password" class="mb-3 text-right text-sm font-medium text-black"
		>{$_('common.password')}؟</Label
	>
	<Input
		class="h-[52px] w-[364px]"
		id="password"
		name="password"
		on:change={handleChange}
		on:blur={handleChange}
		bind:value={$form.password}
		placeholder={$_('common.password')}
		type="password"
	/>
	{#if $errors.password}
		<p class="mt-2 text-sm text-red-500">{$errors.password}</p>
	{/if}
	<div class="mb-2 mt-2 flex w-full flex-row items-center justify-between">
		<p class="text-right text-sm font-medium text-black">{$_('onboarding.singInWith')}</p>
		<Button variant="link"
			><a href="/auth/local_sign_in">
				{$_('onboarding.anotherAccount')}
			</a></Button
		>
	</div>
	<Button type="submit" class="w-full">{$_('onboarding.signIn')}</Button>
	<div class="mt-3 flex flex-row items-center justify-center">
		<Button type="link" variant="link"
			><a href="/auth/local_sign_in/forget_password_local_sign_in">
				{$_('onboarding.forgetPassword')}
			</a></Button
		>
	</div>
</form>
