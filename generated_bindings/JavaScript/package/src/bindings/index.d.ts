// @ts-ignore
import { Rustfft as _Rustfft } from "./rustfft/rustfft";

/**
 * Options used when initializing the bindings.
 */
export type LoadOptions = {
    /** Additional imports to be provided to the WebAssembly module */
    imports: WebAssembly.Imports,
    /**
     * A user-specified WebAssembly module to use instead of the one bundled
     * with this package.
     */
    module: WebAssembly.Module,
};

export default class Bindings {
    rustfft(options?: Partial<LoadOptions>): Promise<_Rustfft>;
}