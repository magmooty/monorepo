<script lang="ts">
	import { Heading, Button, Listgroup, Avatar } from 'flowbite-svelte';
	import { ExclamationCircleSolid } from 'flowbite-svelte-icons';

	import { _ } from 'svelte-i18n';
	import UserSelector from './UserSelector.svelte';
</script>

<div class=" flex h-screen flex-row items-center text-center">
	<div class="flex h-screen w-[50%] flex-col items-center justify-center">
		<div class=" flex flex-col items-center justify-center">
			<div class="">
				<img class="h-[100px] w-[100px]" src="images/logo.png" alt="logo" />
			</div>
			<div class="">
				<Heading tag="h1" class="text-4xl text-[#0B8CD2]">{$_('localSignIn.welcomeAgain')}</Heading>
				<Heading tag="h1" class="text-4xl  text-[#0B8CD2]"
					>{$_('localSignIn.chooseAccountToSignIn')}</Heading
				>
			</div>
		</div>

		<div class="flex w-[274px] flex-col items-center justify-between">
			<UserSelector />

			<div class="h-[93px] w-[274px]">
				<Button class="mb-[20px] w-[274px] bg-[#0B8CD2] hover:bg-[#1EA8F3]">
					{$_('localSignIn.connectToMaster')}
				</Button>
				<a href="/your-link" class="text-[#0B8CD2] underline hover:text-[#1EA8F3]"
					>{$_('common.connectUs')}</a
				>
			</div>
			<Button
				class="h-[48px] w-[274px] border  border-[gray/200] bg-transparent text-black hover:bg-gray-200"
			>
				<ExclamationCircleSolid class="ml-2 mr-2 h-5 w-5 text-black " />
				{$_('localSignIn.recoverData')}
			</Button>
		</div>
	</div>
	<div class="flex h-screen w-[50%] flex-col items-center justify-center bg-[#0B8CD266]">
		<img src="images/cuate.png" alt="login local" />
	</div>
</div>
