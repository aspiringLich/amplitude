import { browser, dev } from '$app/environment';
import { toast } from 'svelte-sonner';

class RequestClient {
	async request(url: string, method: string, data?: any, init?: RequestInit) {
		let opts = { method, ...init };
		if (data) opts.body = JSON.stringify(data);
		const req = new Request(url, opts);
		if (data) req.headers.set('Content-Type', 'application/json');
		const response = await fetch(req);
		if (!response.ok) {
			let opts = {} as any;
			if (response.bodyUsed) opts.description = (await response.text()).split('\n', 2)[0];
			toast.error(`Error ${response.status}: ${response.statusText}`, opts);
		}
		return response;
	}

	async get(url: string) {
		return this.request(url, 'GET', null);
	}

	async post(url: string, data: any) {
		return this.request(url, 'POST', data);
	}

	async put(url: string, data: any) {
		return this.request(url, 'PUT', data);
	}

	async delete(url: string) {
		return this.request(url, 'DELETE', null);
	}
}

export const request = new RequestClient();
