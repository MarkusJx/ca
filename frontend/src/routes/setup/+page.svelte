<script lang="ts">
	import Center from '$lib/components/Center.svelte';
	import Paper from '@smui/paper';
	import Stepper from '$lib/components/stepper/Stepper.svelte';
	import Step from '$lib/components/stepper/Step.svelte';
	import Typography from '$lib/components/Typography.svelte';
	import StepperActions from '$lib/components/stepper/StepperActions.svelte';
	import StepperButton from '$lib/components/stepper/StepperButton.svelte';
	import CircularProgress from '@smui/circular-progress';
	import {
		generateIntermediate,
		generateRootCertificate,
	} from '$lib/api/certificates/certificates';
	import toast from 'svelte-french-toast';
	import type IStep from '$lib/components/stepper/IStep';
	import Button from '@smui/button';
	import type { CACertificateDto } from '$lib/api/models';
	import SetupHeading from './SetupHeading.svelte';
	import SetupContent from './SetupContent.svelte';
	import DownloadCertificateButton from './DownloadCertificateButton.svelte';

	interface Certificates {
		root: CACertificateDto;
		intermediate: CACertificateDto;
	}

	const steps: IStep[] = [
		{ text: 'Introduction' },
		{ text: 'Certificate generation' },
		{ text: 'Done' },
	];

	let currentStep = 0;
	let error = false;
	let certificates: Certificates | null = null;

	const generateCertificates = async (): Promise<Certificates | null> => {
		try {
			const root = await generateRootCertificate();
			const intermediate = await generateIntermediate({
				rootCertificate: root.privateKey!,
			});

			return {
				root,
				intermediate,
			};
		} catch (e) {
			console.error(e);
			toast.error('Failed to generate certificates');
			for (let i = 1; i < steps.length; i++) {
				steps[i].alert = true;
			}
			error = true;
			steps[2].text = 'Error';
		} finally {
			currentStep = steps.length;
		}

		return null;
	};

	$: if (currentStep === 1) {
		generateCertificates().then((certs) => {
			certificates = certs;
		});
	}
</script>

<Center>
	<Paper style="width: 738px">
		<Stepper
			{steps}
			bind:current={currentStep}
			divider
			primary={error ? '#dc3545' : undefined}
			line="2px"
			size="2em"
		>
			<Step current={currentStep} step={0}>
				<SetupHeading>Setup</SetupHeading>
				<Typography style="text-align: center">
					The root and intermediate certificate are not present in the database
					and must be generated. The root certificate is self-signed and the
					intermediate certificate is signed by the root certificate.
					<br /><br />
					The root certificate will not be stored in the database and must be downloaded
					and kept safe by the user.
				</Typography>
			</Step>
			<Step current={currentStep} step={1}>
				<SetupHeading>Generating certificates</SetupHeading>
				<SetupContent>This may take a while...</SetupContent>
				<CircularProgress indeterminate style="height: 50px; width: 50px" />
			</Step>
			<Step current={currentStep} step={[2, 3]}>
				{#if error}
					<SetupHeading>Failed to generate certificates</SetupHeading>
					<SetupContent>
						An error occurred while generating the certificates.
						<br />
						Click the button below to try again.
					</SetupContent>
				{:else}
					<SetupHeading>Download certificates</SetupHeading>
					<SetupContent>
						Download the root certificate and the intermediate certificate.
						<br />
						The root certificate is self-signed and the intermediate certificate
						is signed by the root certificate.
					</SetupContent>
					{#if certificates}
						<DownloadCertificateButton
							content={certificates.root.certificate}
							download="root.crt"
						>
							Download root certificate
						</DownloadCertificateButton>
						<DownloadCertificateButton
							content={certificates.root.privateKey}
							download="root.pem"
						>
							Download root private key
						</DownloadCertificateButton>
						<DownloadCertificateButton
							content={certificates.intermediate.certificate}
							download="intermediate.crt"
						>
							Download intermediate certificate
						</DownloadCertificateButton>
					{/if}
				{/if}
			</Step>
			<StepperActions>
				{#if currentStep < steps.length - 1}
					<StepperButton
						bind:current={currentStep}
						numSteps={steps.length}
						disabled={currentStep === 1}
					>
						Next
					</StepperButton>
				{:else}
					<Button on:click={() => (location.href = '/')}>
						{#if error}
							Retry
						{:else}
							Finish
						{/if}
					</Button>
				{/if}
			</StepperActions>
		</Stepper>
	</Paper>
</Center>
