import { z, ZodError } from 'zod';

export const longStringSchema = z.string().max(6000);

export type ValidationResult<T> =
	| { ok: false; error: ZodError | undefined; status: number | undefined }
	| { ok: true; result: T };

export const typeSchema = z
	.string()
	.min(1)
	.max(30)
	.regex(
		/^(bool|int|float|string)(?:\[\])*$|^(\w+)<(bool|int|float|string)>$/,
		'Invalid type syntax'
	); // TODO: match capturing groups & dynamically validate with language config

export const valid_types = (lang: string | undefined) => {
	// TODO: actual flexibility with language types?? who knows??
	return [
		'bool', 'int', 'float', 'string'
	]
};