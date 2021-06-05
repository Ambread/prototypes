import * as op from "./op";
import * as util from "./util";

const fact = (ret: (it: number) => never) => (n: number): never => 
    op.equal(op.when(
        () => ret(1),
        () => op.subtract(fact(op.multiply(ret)(n)))(n)(1),
    ))(n)(0);

const main = fact(util.log(util.exit));

const _: never = main(10);
