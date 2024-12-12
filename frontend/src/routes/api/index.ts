import { request } from '$src/lib/request';
import * as exec from '$src/routes/api/exec';
import type { ValidationResult } from '$src/routes/api/schema';
import { toast } from 'svelte-sonner';
import { z, type ZodType } from 'zod';

export { exec };

export const api_post = async <Res>(
	url: string,
	reqSchema: ZodType<any, any, any>,
    req: z.infer<typeof reqSchema>
): Promise<ValidationResult<Res>> => {
	const parse = await reqSchema.safeParseAsync(req);
	let error, status;
	if (parse.success) {
		const res = await request.post(url, parse.data);
		if (res.ok) {
			return { ok: true, result: await res.json() };
		} else status = res.status;
	} else {
		error = parse.error;
		console.error(error);
		toast.error('Could not validate request data', { description: `${error.message}` });
	}

	return { ok: false, error, status };
};

