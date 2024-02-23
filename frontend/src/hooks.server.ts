import type { Handle } from '@sveltejs/kit';

// export const handle: Handle = async ({ event, resolve }) => {
// 	if (event.url.pathname.startsWith('/api')) {
// 		const request = new Request(`http://127.0.0.1:3000${event.url.pathname}`, event.request);
// 		const response = await fetch(request);
// 		return response;
// 	}

// 	const response = await resolve(event);
// 	return response;
// };
