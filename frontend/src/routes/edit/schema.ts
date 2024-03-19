import type { Infer, SuperValidated } from 'sveltekit-superforms';
import { z, type RefinementCtx } from 'zod';

const sanitize_html = async (arg: string, _: RefinementCtx) => {
	const sanitizeHtml = await import('sanitize-html');
	return sanitizeHtml.default(arg);
};

const literalSchema = z.union([
	z.string().max(1000, 'strings may not be longer than 1000 bytes'),
	z.number(),
	z.boolean(),
	z.null(),
]);
type Literal = z.infer<typeof literalSchema>;

export const exerciseSchema = z.object({
	title: z.string().trim().min(5).max(32),
	description: z.string().max(6000).superRefine(sanitize_html),
	generator: z.string().max(6000),
	generated_table: z.array(literalSchema).max(100),
	
	starting_code: z.string().max(6000)
});

export type ExerciseScheme = typeof exerciseSchema;
export type Exercise = SuperValidated<Infer<ExerciseScheme>>['data'];
