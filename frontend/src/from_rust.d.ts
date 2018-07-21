/* tslint:disable */
export enum CameraType {Single,FPS,Free,}
export function scene_lib(): any;

export function camera(): any;

export function view_mat(arg0: Float32Array): Float32Array;

export function model_mat(arg0: Float32Array, arg1: number): Float32Array;

export function rotator(arg0: Float32Array): Float32Array;

export class MeshBg {
free(): void;
}
export class LightingBg {
free(): void;
}
export class ShapeBg {
free(): void;
}
export class LightSourceBg {
free(): void;
}
export class CameraBg {
free(): void;
}
