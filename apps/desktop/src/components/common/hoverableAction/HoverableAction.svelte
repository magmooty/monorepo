<script lang="ts">
	export let imageSrc: string;

	if (imageSrc === undefined) {
		throw new Error("The 'imageSrc' prop is required.");
	}
</script>

<button
	type="button"
	on:click
	class={`flex w-1/2 cursor-pointer flex-col items-center justify-center rounded-md p-2 text-center text-primary hover:bg-secondary ${$$props.class}`}
>
	<img src={imageSrc} class="w-full" alt="hoverable_action_image" />
	<slot />
</button>
