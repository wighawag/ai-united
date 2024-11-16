import { writable } from 'svelte/store';

const _files = writable<{ one?: Uint8Array; two?: Uint8Array }>({});

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

async function addFile(index: 0 | 1, file: File) {
	const data = await handleFile(file);
	if (index == 0) {
		_files.update((v) => ({ ...v, one: data }));
	} else {
		_files.update((v) => ({ ...v, two: data }));
	}
}

export const files = {
	subscribe: _files.subscribe,
	addFile
};
