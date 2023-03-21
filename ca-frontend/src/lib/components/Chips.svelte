<script lang="ts">
	import Chip, { Set, Text } from '@smui/chips';

	export let chips: string[];
	export let nonInteractive: boolean = false;
	export let setStyle: string = '';
</script>

<Set {chips} let:chip {nonInteractive} style={setStyle}>
	<Chip {chip}>
		<Text>{chip}</Text>
	</Chip>
</Set>
