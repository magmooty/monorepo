<script>
	import SetSignInLocalPassword from 'components/localSignIn/SetSignInLocalPassword.svelte';
	import { Card } from 'flowbite-svelte';
</script>

<div class="flex h-screen w-full flex-col items-center justify-center">
	<Card
		class="flex max-h-[340px] max-w-[600px] flex-col items-center justify-center bg-[#FBFBFB] shadow-none "
	>
		<div class="max-h-[237px] max-w-[364px]">
			<SetSignInLocalPassword />
		</div>
	</Card>
</div>
