import { Cordinate } from "./__generated__/graphql";

export function isValid(cord: Cordinate): boolean {
  return cord.q + cord.r + cord.s === 0;
}

export function toPixelFlat(cord: Cordinate): [number, number] {
  let x = 1.5 * cord.q;
  let y = (Math.sqrt(3) / 2) * cord.q + Math.sqrt(3) * cord.r;
  return [x, y];
}

export function toPixelPoint(cord: Cordinate): [number, number] {
  let x = Math.sqrt(3) * cord.q + (Math.sqrt(3) / 2) * cord.r;
  let y = 1.5 * cord.r;
  return [x, y];
}
