export interface Model {
    repoId: string;
    label: string;
}

export const v3Models: Model[] = [
    {
        repoId: "SmilingWolf/wd-eva02-large-tagger-v3",
        label: "wd-tagger v3 eva02-large",
    },
    {
        repoId: "SmilingWolf/wd-vit-large-tagger-v3",
        label: "wd-tagger v3 vit-large",
    },
    {
        repoId: "SmilingWolf/wd-swinv2-tagger-v3",
        label: "wd-tagger v3 swin-v2",
    },
    {
        repoId: "SmilingWolf/wd-convnext-tagger-v3",
        label: "wd-tagger v3 convnext",
    },
    {
        repoId: "SmilingWolf/wd-vit-tagger-v3",
        label: "wd-tagger v3 vit",
    },
];

export const deepghsModels: Model[] = [
    {
        repoId: "deepghs/idolsankaku-eva02-large-tagger-v1",
        label: "idolsankaku v1 eva02-large",
    },
    {
        repoId: "deepghs/idolsankaku-swinv2-tagger-v1",
        label: "idolsankaku v1 swin-v2",
    },
];

export const getModels = (): Model[] => {
    return [...v3Models, ...deepghsModels];
};
export const getDefaultModel = (): Model => {
    return v3Models[0];
};
