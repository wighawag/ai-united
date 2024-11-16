import { writable } from 'svelte/store';

const _files = writable<Uint8Array[]>([]);

function handleFile(file: File): Promise<Uint8Array> {
	return new Promise((resolve, reject) => {
		const reader = new FileReader();

		reader.onload = (event) => {
			if (event.target && event.target.result) {
				resolve(new Uint8Array(event.target.result as ArrayBuffer)); // TODO handle string type ?
			} else {
				reject(new Error('Failed to read file'));
			}
		};

		reader.onerror = (error) => {
			reject(error);
		};

		reader.readAsArrayBuffer(file);
	});
}

async function addFiles(filesGiven: File[]) {
	for (const file of filesGiven) {
		const data = await handleFile(file);
		_files.update((v) => [...v, data]);
	}
}

export const files = {
	subscribe: _files.subscribe,
	addFiles
};
