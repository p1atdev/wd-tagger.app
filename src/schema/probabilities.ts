import { type } from "arktype";

export const probabilitiesType = type({
    "[string]": "number",
});

// extract the type if needed
export type Probabilities = typeof probabilitiesType.infer;
