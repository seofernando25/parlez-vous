export interface Theme {
    id: string;
    name: string;
    description: string;
    subthemes: string[];
}

export interface Tier {
    level: number;
    cefr: string;
    name: string;
    description: string;
    color: string;
    xpThreshold: number;
    themes: Theme[];
}

export const XP_THRESHOLDS = {
    1: 0, 2: 36000, 3: 144000, 4: 360000, 5: 720000, 6: 1440000, 7: 2880000
} as const;
