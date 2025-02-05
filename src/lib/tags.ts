import type { Probabilities } from "@/schema/probabilities";

export const filterHighestProbabilityTag = (
    tagMap: Probabilities,
): Probabilities => {
    const highestProbability = Math.max(...Object.values(tagMap));
    return Object.fromEntries(
        Object.entries(tagMap).filter(([_, probability]) =>
            probability === highestProbability
        ),
    );
};

export const filterTagsByThreshold = (
    tagMap: Probabilities,
    threshold: number,
): Probabilities => {
    return Object.fromEntries(
        Object.entries(tagMap)
            .filter(([_, probability]) => probability >= threshold)
            .sort(([_1, prob1], [_2, prob2]) => prob2 - prob1),
    );
};
