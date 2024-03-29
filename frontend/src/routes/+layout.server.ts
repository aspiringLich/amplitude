export type AvatarData = { name: string; avatar_url: string };

export const load = async ({ fetch, cookies }) => {
	if (cookies.get('session')) {
		const res = await fetch('/api/auth/session', { method: 'GET' });
		if (res.ok) {
			const data: AvatarData = await res.json();
			return { avatar: data };
		}
	}
};
