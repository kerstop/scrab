export class Cordinate {
  valid: boolean;
  q: number;
  r: number;
  s: number;
  constructor(q: number, r: number, s: number) {
    if (q + r + s == 0) {
      this.valid = true;
      this.q = q;
      this.r = r;
      this.s = s;
    } else {
      this.valid = false;
      this.q = NaN;
      this.r = NaN;
      this.s = NaN;
    }
  }

  isValid() {
    this.valid
  }

  toPixelFlat(): [number,number] {
    let x = 1.5 * this.q;
    let y = Math.sqrt(3) / 2 * this.q + Math.sqrt(3) * this.r
    return [x,y]
  }

  toPixelPoint(): [number,number] {
    let x = Math.sqrt(3) * this.q + Math.sqrt(3) / 2 * this.r;
    let y = 1.5 * this.r
    return [x,y]
  }
}
