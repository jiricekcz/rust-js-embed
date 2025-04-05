import { decrement } from './test.js';
const Vec2D = Deno.core.ops.Vec2D;
export function invert(n) {
    let acc = 0;
    console.log(Vec2D.get_x(new Vec2D(1, 2)));
    for (let i = 0; i < n; i++) {

        acc = decrement(acc);
    }
    return acc;
}