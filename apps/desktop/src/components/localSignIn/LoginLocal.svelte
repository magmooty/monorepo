<script lang="ts">
	import { Heading, Button, Listgroup, Avatar } from 'flowbite-svelte';
	import { ExclamationCircleSolid } from 'flowbite-svelte-icons';
	import { app } from 'sdk';
	import type { PublicUserInfo } from 'sdk/auth';

	import { _ } from 'svelte-i18n';
	let users: PublicUserInfo[] = [];
	async function fetchUsers() {
		try {
			const fetchedUsers = await app.auth.listUsers();
			users = fetchedUsers;
		} catch (error) {
			//TODO: Handle error
			console.error('Failed to fetch users:', error);
		}
	}
	console.log('users:', users);
	fetchUsers();
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
			<div class=" h-[365px] overflow-y-auto">
				{#if users.length > 0}
					<Listgroup items={users} let:item class="border-0 p-0 dark:!bg-transparent">
						<div
							class="flex h-[73px] w-[274px] items-center space-x-2 hover:bg-gray-200 rtl:space-x-reverse"
						>
							<Avatar alt="logo" class="flex-shrink-0" />
							<div class="min-w-0 flex-1 ltr:text-left rtl:text-right">
								<p class="font-siz-[16px] truncate text-gray-900 dark:text-white">
									{item.name}
								</p>
								<p>
									{#if item.is_center_manager && item.manages_spaces.length > 0}
										{$_('common.teacher')} {$_('common.and')} {$_('common.manager')}
									{:else if item.manages_spaces.length > 0}
										{$_('common.teacher')}
									{:else}
										{$_('common.assistantIn') + ' '}
										{#each item.member_of_spaces as space, index}
											{space}
											{#if index < item.member_of_spaces.length - 1}
												{$_('common.and')}
											{/if}
										{/each}
									{/if}
								</p>
							</div>
						</div>
					</Listgroup>
				{:else}
					<p>
						{$_('common.loading')}
					</p>
				{/if}
			</div>

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
