import type { Infer, SuperValidated } from 'sveltekit-superforms';
import { z, type RefinementCtx } from 'zod';

export const sanitize_html = async (arg: string, _: RefinementCtx) => {
	const sanitizeHtml = await import('sanitize-html');
	return sanitizeHtml.default(arg);
}
export const exerciseSchema = z.object({
	title: z.string().trim().min(5).max(32),
	description: z.string().max(6000).superRefine(sanitize_html),
	generator: z.string().max(6000),
	starting_code: z.string().max(6000)
});

export type ExerciseScheme = typeof exerciseSchema;
export type Exercise = SuperValidated<Infer<ExerciseScheme>>["data"];