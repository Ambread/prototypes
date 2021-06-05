export const exit = (): never => process.exit();

export const log = (ret: () => never) => (message: any): never => {
    console.log(message);
    ret();
};
