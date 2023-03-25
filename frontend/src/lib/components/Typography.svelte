<script lang="ts">
	export let variant: string = 'p';
	export let style: string | undefined = undefined;

	const getVariant = () => {
		switch (variant) {
			case 'h1':
				return 'headline1';
			case 'h2':
				return 'headline2';
			case 'h3':
				return 'headline3';
			case 'h4':
				return 'headline4';
			case 'h5':
				return 'headline5';
			case 'h6':
				return 'headline6';
			case 'p':
				return 'body1';
			default:
				throw new Error('Invalid variant');
		}
	};
</script>

<div class={`mdc-typography--${getVariant()} `} {style}>
	<slot />
</div>

<style lang="scss">
	@use '../styles/typography.scss';
</style>
