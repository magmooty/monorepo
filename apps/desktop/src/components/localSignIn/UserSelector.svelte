<script lang="ts">
	import { Listgroup, Avatar } from 'flowbite-svelte';
	import type { PublicUserInfo, LocalUserWithPermissionsToResetPassword } from 'sdk/auth';
	import { _ } from 'svelte-i18n';
	export let users: PublicUserInfo[] | LocalUserWithPermissionsToResetPassword[];
	export let showSubtext: boolean = false;
</script>

<div class="max-h-[365px] overflow-y-auto">
	{#if users.length > 0}
		<Listgroup items={users} let:item class="border-0 p-0 dark:!bg-transparent">
			<div
				class="flex h-[73px] w-[274px] items-center space-x-2 hover:bg-gray-200 rtl:space-x-reverse"
			>
				<Avatar alt="logo" class="flex-shrink-0" />
				<div class="min-w-0 flex-1 ltr:text-left rtl:text-right">
					<p class="font-size-[16px] truncate text-gray-900 dark:text-white">
						{item.name}
					</p>
					{#if showSubtext}
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
					{/if}
				</div>
			</div>
		</Listgroup>
	{:else}
		<p>
			{$_('common.loading')}
		</p>
	{/if}
</div>
