import { names, langs } from '$src/lib/components/editor/lang';
import type { Infer, SuperValidated } from 'sveltekit-superforms';
import { z, type RefinementCtx } from 'zod';

const sanitize_html = async (arg: string, _: RefinementCtx) => {
	const sanitizeHtml = await import('sanitize-html');
	return sanitizeHtml.default(arg);
};
// TODO: move this to a util file and make sure its used when rendering with @html

const literalSchema = z.union([
	z.string().max(1000, 'strings may not be longer than 1000 bytes'),
	z.number(),
	z.boolean(),
	z.null()
]);
type Literal = z.infer<typeof literalSchema>;

const langSchema = z.enum(names);

export const exerciseSchema = z.object({
	title: z.string().trim().min(5).max(32),
	description: z.string().max(6000).superRefine(sanitize_html),
	// generator_lang: langSchema.optional(),
	// generator: z.string().max(6000).optional(),
	generated_table: z.array(literalSchema).max(100),
	starting_code: z.string().max(6000)
});

export type ExerciseSchema = typeof exerciseSchema;
export type Exercise = SuperValidated<Infer<ExerciseSchema>>['data'];
