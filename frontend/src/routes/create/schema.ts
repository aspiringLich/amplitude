import { z } from 'zod';

export type Exercise = {
	name: string;
	description?: string;
	generator?: string;
	starting_code?: string;
};

export const exerciseSchema = z.object({
	name: z
		.string()
		.trim()
		.min(5)
		.max(32)
		.refine(/^([\w\d]+ )*[\w\d]+$/.test, {
			message: '`name` must be alphanumeric characters seperated by single spaces'
		}),
	description: z.string().max(6000),
	generator: z.string().max(6000),
	starting_code: z.string().max(6000),
});
