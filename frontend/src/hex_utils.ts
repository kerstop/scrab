export interface Cordinate {
  q:number,
  r:number,
  s:number,
}

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

export const UNIT_CORDINATES: {
  readonly qr: Cordinate;
  readonly qs: Cordinate;
  readonly rq: Cordinate;
  readonly rs: Cordinate;
  readonly sq: Cordinate;
  readonly sr: Cordinate;
} = {
  qr: { q: 1, r: -1, s: 0 },
  qs: { q: 1, r: 0, s: -1 },
  rq: { q: -1, r: 1, s: 0 },
  rs: { q: 0, r: 1, s: -1 },
  sq: { q: -1, r: 0, s: 1 },
  sr: { q: 0, r: -1, s: 1 },
};
