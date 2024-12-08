import { names, langs } from '$src/lib/components/editor/lang';
import type { Infer, SuperValidated } from 'sveltekit-superforms';
import { z, type RefinementCtx } from 'zod';

const typeSchema = z
	.string()
	.min(1)
	.max(30)
	.regex(
		/^(bool|int|float|string)(?:\[\])*$|^(\w+)<(bool|int|float|string)>$/,
		'Invalid type syntax'
	); // TODO: match capturing groups & dynamically validate with language config

const sanitize_html = async (arg: string, _: RefinementCtx) => {
	const sanitizeHtml = await import('sanitize-html');
	return sanitizeHtml.default(arg);
};
// TODO: move this to a util file and make sure its used when rendering with @html

const literalSchema = z.union([
	z.string().max(1000, 'Strings may not be longer than 1000 bytes'),
	z.number(),
	z.boolean(),
	z.null()
]);
type Literal = z.infer<typeof literalSchema>;

const langSchema = z.enum(names);

export const exerciseSchema = z.object({
	title: z.string().trim().min(5).max(32),
	description: z.string().min(20).max(6000).superRefine(sanitize_html),
	function_name: z
		.string()
		.min(1)
		.max(50)
		.regex(/[\w_][\w\d_]*/, 'Function name must be composed of characters [a-zA-Z0-9_]'),
	inputs: z.array(typeSchema).max(8),
	outputs: typeSchema,
	starting_code: z.string().max(6000).optional(),

	generator_lang: langSchema
		.refine((lang) => langs[lang].type == 'scripting', 'Language must be a scripting language')
		.optional(),
	generator: z.string().max(6000).optional(),
	generated_table: z.array(literalSchema).max(500)
});

export type ExerciseSchema = typeof exerciseSchema;
export type Exercise = SuperValidated<Infer<ExerciseSchema>>['data'];
