export function capitalizeFirstLetter(str: string): string {
	return str.charAt(0).toUpperCase() + str.slice(1);
}

export function downloadFile(data: BlobPart[], filename: string): void {
	let url: string | null = null;
	let link: HTMLAnchorElement | null = null;

	try {
		const blob = new Blob(data);
		url = URL.createObjectURL(blob);
		link = document.createElement('a');
		link.style.display = 'none';
		link.href = url;
		link.download = filename;
		document.body.appendChild(link);
		link.click();
	} finally {
		if (link) document.body.removeChild(link);
		if (url) URL.revokeObjectURL(url);
	}
}

export function onVisible<T extends Element>(
	element: T,
	callback: (el: T) => void
): void {
	new IntersectionObserver((entries, observer) => {
		entries.forEach((entry) => {
			if (entry.intersectionRatio > 0) {
				callback(element);
				observer.disconnect();
			}
		});
	}).observe(element);
}
