import type { Infer, SuperValidated } from 'sveltekit-superforms';
import { z } from 'zod';

export const exerciseSchema = z.object({
	title: z
		.string()
		.trim()
		.min(5)
		.max(32),
	description: z.string().max(6000),
	generator: z.string().max(6000),
	starting_code: z.string().max(6000),
});

export type ExerciseScheme = typeof exerciseSchema;
export type Exercise = SuperValidated<Infer<ExerciseScheme>>["data"];