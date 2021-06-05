type Unit = () => never;
type Ret<T> = (it: T) => never;

export const when = (retTrue: Unit, retFalse: Unit) => (bool: boolean): never => bool ? retTrue() : retFalse();

export const equal = <T>(ret: Ret<boolean>) => (x: T) => (y: T): never => ret(x === y);
export const notEqual = <T>(ret: Ret<boolean>) => (x: T) => (y: T): never => ret(x !== y);

export const add = <T>(ret: Ret<number>) => (x: number) => (y: number): never => ret(x + y);
export const subtract = <T>(ret: Ret<number>) => (x: number) => (y: number): never => ret(x - y);
export const multiply = <T>(ret: Ret<number>) => (x: number) => (y: number): never => ret(x * y);
export const divide = <T>(ret: Ret<number>) => (x: number) => (y: number): never => ret(x / y);

export const not = (ret: Ret<boolean>) => (x: boolean): never => ret(!x);
export const and = (ret: Ret<boolean>) => (x: boolean) => (y: boolean): never => ret(x && y);
export const or = (ret: Ret<boolean>) => (x: boolean) => (y: boolean): never => ret(x || y);
