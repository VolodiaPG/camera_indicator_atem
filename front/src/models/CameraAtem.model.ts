export class CameraAtem {
  preview: number;
  air: number;
}

export let cameraAtemGuard = (val: unknown): val is CameraAtem => {
  return (
    val !== undefined &&
    val !== null &&
    val["preview"] !== undefined &&
    val["air"] !== undefined
  );
};
