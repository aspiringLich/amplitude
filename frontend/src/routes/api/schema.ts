import { z, ZodError } from 'zod';

export const longStringSchema = z.string().max(6000);

export type ValidationResult<T> =
	| { ok: false; error: ZodError | undefined; status: number | undefined }
	| { ok: true; result: T };
