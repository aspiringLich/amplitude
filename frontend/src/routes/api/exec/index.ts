import { request } from '$src/lib/request';
import { api_post } from '$src/routes/api';
import { functionArgsSchema, generatorLangSchema, typeSchema } from '$src/routes/api/exec/schema';
import { longStringSchema, type ValidationResult } from '$src/routes/api/schema';
import { z } from 'zod';

const genRequest = z.object({
	content: longStringSchema,
	language: generatorLangSchema,
	inputs: functionArgsSchema,
	output: typeSchema,
	visible_cases: z.number().int().positive(),
	hidden_cases: z.number().int().positive(),
	generate_cases: z.number().int().positive()
});
type GenResult =
	| {
			exit_code: undefined;
			cases: { input: any[]; output: any }[];
			stderr: string;
			stdout: string;
	  }
	| {
			exit_code: number;
			stderr: string;
			stdout: string;
	  };

export const gen = async (req: z.infer<typeof genRequest>) => {
    return api_post<GenResult>('/api/exec/gen', genRequest, req)
}
