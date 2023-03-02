const { bindings } = require("@dynamite-bud/rustfft");

const {
  Complex,
  Algorithm,
} = require("@dynamite-bud/rustfft/src/bindings/rustfft/rustfft.js");

const { test, expect } = require("@jest/globals");

test("dft test len 3", async () => {
  const wasm = await bindings.rustfft();
  let signal = [
    { re: 1.0, im: 1.0 },
    {
      re: 2.0,
      im: -3.0,
    },
    {
      re: -1.0,
      im: 4.0,
    },
  ];
  let spectrum = [
    { re: 2.0, im: 2.0 },
    {
      re: -5.562177,
      im: -2.098076,
    },
    {
      re: 6.562178,
      im: 3.09807,
    },
  ];
  let dft = Algorithm.newDft(wasm, signal.length, "forward");
  let output = dft.compute(signal);
  output.forEach((element, index) => {
    expect(element.re.toPrecision(5)).toBe(spectrum[index].re.toPrecision(5));
  });
});

test("dft test len 4", async () => {
  const wasm = await bindings.rustfft();
  let signal = [
    { re: 0.0, im: 1.0 },
    {
      re: 2.5,
      im: -3.0,
    },
    {
      re: -1.0,
      im: -1.0,
    },
    { re: 4.0, im: 0.0 },
  ];
  let spectrum = [
    {
      re: 5.5,
      im: -3.0,
    },
    {
      re: -2.0,
      im: 3.5,
    },
    {
      re: -7.5,
      im: 3.0,
    },
    {
      re: 4.0,
      im: 0.5,
    },
  ];
  let dft = Algorithm.newDft(wasm, signal.length, "forward");
  let output = dft.compute(signal);
  output.forEach((element, index) => {
    expect(element.re.toPrecision(2)).toBe(spectrum[index].re.toPrecision(2));
  });
});
