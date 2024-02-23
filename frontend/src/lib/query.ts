import { browser, dev } from "$app/environment";

class RequestClient {
	async request(url: string, method: string, data?: any, init?: RequestInit) {
		const req = new Request(url, { method, body: JSON.stringify(data), ...init });
		if (data)
			req.headers.set('Content-Type', 'application/json');
		const response = await fetch(req);
		if (!response.ok) throw new Error(response.statusText);
		return response;
	}
	
	async get(url: string) {
		return (await this.request(url, 'GET', null)).json();
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