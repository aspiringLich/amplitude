class QueryClient {
	async get(url: string) {
		const response = await fetch(url, { method: 'GET' });
		if (!response.ok) throw new Error(response.statusText);
		return response.json();
	}

	async post(url: string, data: any) {
		const response = await fetch(url, { method: 'POST', body: JSON.stringify(data) });
		if (!response.ok) throw new Error(response.statusText);
		return response.json();
	}

	async put(url: string, data: any) {
		const response = await fetch(url, { method: 'PUT', body: JSON.stringify(data) });
		if (!response.ok) throw new Error(response.statusText);
		return response.json();
	}

	async delete(url: string) {
		const response = await fetch(url, { method: 'DELETE' });
		if (!response.ok) throw new Error(response.statusText);
		return response.json();
	}
}

export const query = new QueryClient();