<script>
	import Button from 'components/common/button/Button.svelte';
	import HoverableAction from 'components/common/hoverableAction/HoverableAction.svelte';
	import { ExclamationCircleSolid } from 'flowbite-svelte-icons';
</script>

<h1>Home</h1>

<HoverableAction imageSrc="images/connect_to_master.svg" class="m-3">
	<span class="jusitfy-center m-3 flex items-center">
		<ExclamationCircleSolid class="m-1" />
		Connect to master
	</span>
</HoverableAction>
