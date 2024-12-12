import { names, langs } from '$src/lib/components/editor/lang';
import { sanitize_html } from '$src/lib/utils';
import { typeSchema } from '../schema';
import { longStringSchema } from '$src/routes/api/schema';
import type { Infer, SuperValidated } from 'sveltekit-superforms';
import { z } from 'zod';

const literalSchema = z.union([
	z.string().max(1000, 'Strings may not be longer than 1000 bytes'),
	z.number(),
	z.boolean(),
	z.null()
]);
type Literal = z.infer<typeof literalSchema>;

export const langSchema = z.enum(names);

export const functionArgsSchema = z.array(typeSchema).min(1).max(8);
export const generatorLangSchema = langSchema.refine(
	(lang) => langs[lang].type == 'scripting',
	'Language must be a scripting language'
);

export const exerciseSchema = z.object({
	title: z.string().trim().min(5).max(32),
	description: longStringSchema.min(20).superRefine(sanitize_html),
	function_name: z
		.string()
		.min(1)
		.max(50)
		.regex(/[\w_][\w\d_]*/, 'Function name must be composed of characters [a-zA-Z0-9_]'),
	input: functionArgsSchema,
	output: typeSchema,
	solution: longStringSchema,
	
	starting_code: longStringSchema.optional(),

	generator_lang: generatorLangSchema.optional(),
	generator: longStringSchema.optional(),
	generated_table: z.array(literalSchema).max(500)
});

export type ExerciseSchema = typeof exerciseSchema;
export type Exercise = SuperValidated<Infer<ExerciseSchema>>['data'];
