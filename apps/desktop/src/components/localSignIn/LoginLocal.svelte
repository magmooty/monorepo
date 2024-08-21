<script lang="ts">
	import { Heading, P, Button, Card, Listgroup, Avatar } from 'flowbite-svelte';
	import { ExclamationCircleSolid } from 'flowbite-svelte-icons';
	import { app } from 'sdk';
	import type { PublicUserInfo } from 'sdk/auth';

	import { _ } from 'svelte-i18n';

	export const mockUsers: PublicUserInfo[] = [
		{
			id: 1,
			name: 'كريم جابر',
			phone_number: '+201096707442',
			member_of_spaces: [],
			manages_spaces: ['كريم جابر'],
			is_center_manager: true
		},
		{
			id: 1,
			name: 'كريم جابر',
			phone_number: '+201096707442',
			member_of_spaces: [],
			manages_spaces: ['كريم جابر'],
			is_center_manager: true
		},
		{
			id: 22,
			name: 'اسماء',
			phone_number: '+201070671580',
			member_of_spaces: ['كريم جابر'],
			manages_spaces: [],
			is_center_manager: false
		},
		{
			id: 3,
			name: 'مصطفى قلقيلة',
			phone_number: '+201151002051',
			member_of_spaces: [],
			manages_spaces: ['مصطفى قلقيلة'],
			is_center_manager: false
		},
		{
			id: 4,
			name: 'طارق',
			phone_number: '+201557873011',
			member_of_spaces: ['مصطفى قلقيلة', 'كريم جابر'],
			manages_spaces: [],
			is_center_manager: false
		}
	];
	async function listUsers() {
		const users = await app.auth.listUsers();
		return users;
	}
	listUsers();
</script>

<div class=" flex h-[1024px] min-h-screen flex-row items-center text-center">
	<div class="flex h-[100%] w-[50%] flex-col items-center justify-center">
		<div class="mb-[45.39px] flex h-[321.66px] flex-col items-center justify-center">
			<div class="mb-[11px] mt-[80px]">
				<img class="h-[120px] w-[120px]" src="images/logo.png" alt="logo" />
			</div>
			<div class="h-[108px]">
				<Heading tag="h6" class="text-[36px] text-[#0B8CD2]"
					>{$_('localSignIn.welcomeAgain')}</Heading
				>
				<Heading tag="h6" class="text-[36px] text-[#0B8CD2]"
					>{$_('localSignIn.chooseAccountToSignIn')}</Heading
				>
			</div>
		</div>

		<div class="max:h-[584.39px] flex w-[274px] flex-col items-center justify-between">
			<div class="h-[365px] overflow-y-auto">
				<Listgroup items={mockUsers} let:item class="border-0 p-0 dark:!bg-transparent">
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
									{$_('common.assistantIn')}
									{#each item.member_of_spaces as space, index}
										{space}
										{#if item.member_of_spaces.length > 1 && index < item.member_of_spaces.length - 1}
											{$_('common.and')}
										{/if}
									{/each}
								{/if}
							</p>
						</div>
					</div>
				</Listgroup>
			</div>

			<div class="mb-[48px] h-[93px] w-[274px]">
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
	<div class="flex h-[100%] w-[50%] flex-col items-center justify-center bg-[#0B8CD266]">
		<img src="images/cuate.png" alt="login local" />
	</div>
</div>
