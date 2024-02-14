import {NativeModules} from 'react-native';

// call our Rust module
const {ExampleJsiModule} = NativeModules;
ExampleJsiModule.install();

type FastDecimal = {
  toString: () => string;
  toNumber: () => number;
  add: (decimal: FastDecimal) => number;
};

declare global {
  // TODO: Add support for number
  var __FastDecimal: (value: string) => FastDecimal;
}

/**
 * Ultra-fast drop-in replacement for Decimal.js written in Rust.
 */
export class Decimal {
  fastDecimal: FastDecimal;

  constructor(value: string) {
    this.fastDecimal = __FastDecimal(value);
  }

  toString(): string {
    return this.fastDecimal.toString();
  }

  toNumber(): number {
    return this.fastDecimal.toNumber();
  }

  // TODO: This takes in a Decimal.Value (string | number | Decimal)
  add(decimal: Decimal): Decimal {
    return this.fastDecimal.add(decimal.fastDecimal);
  }
}
