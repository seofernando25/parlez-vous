export const BUILT_IN_VRMS = ['avatar.vrm', 'man.vrm'] as const;

export type BuiltInVrm = (typeof BUILT_IN_VRMS)[number];
