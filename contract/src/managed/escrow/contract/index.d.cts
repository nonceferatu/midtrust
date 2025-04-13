import type * as __compactRuntime from '@midnight-ntwrk/compact-runtime';

export enum STATE { vacant = 0, occupied = 1 }

export type Witnesses<T> = {
  local_secret_key(context: __compactRuntime.WitnessContext<Ledger, T>): [T, Uint8Array];
}

export type ImpureCircuits<T> = {
  create_deal(context: __compactRuntime.CircuitContext<T>,
              provided_hash_0: Uint8Array): __compactRuntime.CircuitResults<T, []>;
  verify_proof(context: __compactRuntime.CircuitContext<T>,
               provided_secret_hash_0: Uint8Array): __compactRuntime.CircuitResults<T, []>;
}

export type PureCircuits = {
  public_key(sk_0: Uint8Array, instance_0: Uint8Array): Uint8Array;
}

export type Circuits<T> = {
  create_deal(context: __compactRuntime.CircuitContext<T>,
              provided_hash_0: Uint8Array): __compactRuntime.CircuitResults<T, []>;
  verify_proof(context: __compactRuntime.CircuitContext<T>,
               provided_secret_hash_0: Uint8Array): __compactRuntime.CircuitResults<T, []>;
  public_key(context: __compactRuntime.CircuitContext<T>,
             sk_0: Uint8Array,
             instance_0: Uint8Array): __compactRuntime.CircuitResults<T, Uint8Array>;
}

export type Ledger = {
  readonly state: STATE;
  readonly secretHash: { is_some: boolean, value: Uint8Array };
  readonly instance: bigint;
  readonly poster: Uint8Array;
}

export type ContractReferenceLocations = any;

export declare const contractReferenceLocations : ContractReferenceLocations;

export declare class Contract<T, W extends Witnesses<T> = Witnesses<T>> {
  witnesses: W;
  circuits: Circuits<T>;
  impureCircuits: ImpureCircuits<T>;
  constructor(witnesses: W);
  initialState(context: __compactRuntime.ConstructorContext<T>): __compactRuntime.ConstructorResult<T>;
}

export declare function ledger(state: __compactRuntime.StateValue): Ledger;
export declare const pureCircuits: PureCircuits;
