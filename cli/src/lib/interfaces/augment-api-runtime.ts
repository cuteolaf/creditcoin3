// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/calls';

import type { ApiTypes, AugmentedCall, DecoratedCallBase } from '@polkadot/api-base/types';
import type { Bytes, Null, Option, Vec } from '@polkadot/types-codec';
import type { AnyNumber, ITuple } from '@polkadot/types-codec/types';
import type {
    BabeEquivocationProof,
    BabeGenesisConfiguration,
    Epoch,
    OpaqueKeyOwnershipProof,
} from '@polkadot/types/interfaces/babe';
import type { CheckInherentsResult, InherentData } from '@polkadot/types/interfaces/blockbuilder';
import type { BlockHash } from '@polkadot/types/interfaces/chain';
import type { AuthorityId } from '@polkadot/types/interfaces/consensus';
import type { TransactionV2 } from '@polkadot/types/interfaces/eth';
import type { Extrinsic } from '@polkadot/types/interfaces/extrinsics';
import type { AuthorityList, GrandpaEquivocationProof, SetId } from '@polkadot/types/interfaces/grandpa';
import type { AccountId, Block, Header, Index, KeyTypeId, Slot } from '@polkadot/types/interfaces/runtime';
import type { RuntimeVersion } from '@polkadot/types/interfaces/state';
import type { ApplyExtrinsicResult } from '@polkadot/types/interfaces/system';
import type { TransactionSource, TransactionValidity } from '@polkadot/types/interfaces/txqueue';
import type { IExtrinsic, Observable } from '@polkadot/types/types';

export type __AugmentedCall<ApiType extends ApiTypes> = AugmentedCall<ApiType>;
export type __DecoratedCallBase<ApiType extends ApiTypes> = DecoratedCallBase<ApiType>;

declare module '@polkadot/api-base/types/calls' {
    interface AugmentedCalls<ApiType extends ApiTypes> {
        /** 0xbc9d89904f5b923f/1 */
        accountNonceApi: {
            /**
             * The API to query account nonce (aka transaction index)
             **/
            accountNonce: AugmentedCall<ApiType, (accountId: AccountId | string | Uint8Array) => Observable<Index>>;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xcbca25e39f142387/2 */
        babeApi: {
            /**
             * Return the genesis configuration for BABE. The configuration is only read on genesis.
             **/
            configuration: AugmentedCall<ApiType, () => Observable<BabeGenesisConfiguration>>;
            /**
             * Returns information regarding the current epoch.
             **/
            currentEpoch: AugmentedCall<ApiType, () => Observable<Epoch>>;
            /**
             * Returns the slot that started the current epoch.
             **/
            currentEpochStart: AugmentedCall<ApiType, () => Observable<Slot>>;
            /**
             * Generates a proof of key ownership for the given authority in the current epoch.
             **/
            generateKeyOwnershipProof: AugmentedCall<
                ApiType,
                (
                    slot: Slot | AnyNumber | Uint8Array,
                    authorityId: AuthorityId | string | Uint8Array,
                ) => Observable<Option<OpaqueKeyOwnershipProof>>
            >;
            /**
             * Returns information regarding the next epoch (which was already previously announced).
             **/
            nextEpoch: AugmentedCall<ApiType, () => Observable<Epoch>>;
            /**
             * Submits an unsigned extrinsic to report an equivocation.
             **/
            submitReportEquivocationUnsignedExtrinsic: AugmentedCall<
                ApiType,
                (
                    equivocationProof:
                        | BabeEquivocationProof
                        | { offender?: any; slotNumber?: any; firstHeader?: any; secondHeader?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof: OpaqueKeyOwnershipProof | string | Uint8Array,
                ) => Observable<Option<Null>>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0x40fe3ad401f8959a/6 */
        blockBuilder: {
            /**
             * Apply the given extrinsic.
             **/
            applyExtrinsic: AugmentedCall<
                ApiType,
                (extrinsic: Extrinsic | IExtrinsic | string | Uint8Array) => Observable<ApplyExtrinsicResult>
            >;
            /**
             * Check that the inherents are valid.
             **/
            checkInherents: AugmentedCall<
                ApiType,
                (
                    block: Block | { header?: any; extrinsics?: any } | string | Uint8Array,
                    data: InherentData | { data?: any } | string | Uint8Array,
                ) => Observable<CheckInherentsResult>
            >;
            /**
             * Finish the current block.
             **/
            finalizeBlock: AugmentedCall<ApiType, () => Observable<Header>>;
            /**
             * Generate inherent extrinsics.
             **/
            inherentExtrinsics: AugmentedCall<
                ApiType,
                (inherent: InherentData | { data?: any } | string | Uint8Array) => Observable<Vec<Extrinsic>>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xe65b00e46cedd0aa/2 */
        convertTransactionRuntimeApi: {
            /**
             * Converts an Ethereum-style transaction to Extrinsic
             **/
            convertTransaction: AugmentedCall<
                ApiType,
                (
                    transaction:
                        | TransactionV2
                        | { Legacy: any }
                        | { EIP2930: any }
                        | { EIP1559: any }
                        | string
                        | Uint8Array,
                ) => Observable<Extrinsic>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xdf6acb689907609b/4 */
        core: {
            /**
             * Execute the given block.
             **/
            executeBlock: AugmentedCall<
                ApiType,
                (block: Block | { header?: any; extrinsics?: any } | string | Uint8Array) => Observable<Null>
            >;
            /**
             * Initialize a block with the given header.
             **/
            initializeBlock: AugmentedCall<
                ApiType,
                (
                    header:
                        | Header
                        | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any }
                        | string
                        | Uint8Array,
                ) => Observable<Null>
            >;
            /**
             * Returns the version of the runtime.
             **/
            version: AugmentedCall<ApiType, () => Observable<RuntimeVersion>>;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xed99c5acb25eedf5/3 */
        grandpaApi: {
            /**
             * Get current GRANDPA authority set id.
             **/
            currentSetId: AugmentedCall<ApiType, () => Observable<SetId>>;
            /**
             * Generates a proof of key ownership for the given authority in the given set.
             **/
            generateKeyOwnershipProof: AugmentedCall<
                ApiType,
                (
                    setId: SetId | AnyNumber | Uint8Array,
                    authorityId: AuthorityId | string | Uint8Array,
                ) => Observable<Option<OpaqueKeyOwnershipProof>>
            >;
            /**
             * Get the current GRANDPA authorities and weights. This should not change except for when changes are scheduled and the corresponding delay has passed.
             **/
            grandpaAuthorities: AugmentedCall<ApiType, () => Observable<AuthorityList>>;
            /**
             * Submits an unsigned extrinsic to report an equivocation.
             **/
            submitReportEquivocationUnsignedExtrinsic: AugmentedCall<
                ApiType,
                (
                    equivocationProof:
                        | GrandpaEquivocationProof
                        | { setId?: any; equivocation?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof: OpaqueKeyOwnershipProof | string | Uint8Array,
                ) => Observable<Option<Null>>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xf78b278be53f454c/2 */
        offchainWorkerApi: {
            /**
             * Starts the off-chain task for given block header.
             **/
            offchainWorker: AugmentedCall<
                ApiType,
                (
                    header:
                        | Header
                        | { parentHash?: any; number?: any; stateRoot?: any; extrinsicsRoot?: any; digest?: any }
                        | string
                        | Uint8Array,
                ) => Observable<Null>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xab3c0572291feb8b/1 */
        sessionKeys: {
            /**
             * Decode the given public session keys.
             **/
            decodeSessionKeys: AugmentedCall<
                ApiType,
                (encoded: Bytes | string | Uint8Array) => Observable<Option<Vec<ITuple<[Bytes, KeyTypeId]>>>>
            >;
            /**
             * Generate a set of session keys with optionally using the given seed.
             **/
            generateSessionKeys: AugmentedCall<
                ApiType,
                (seed: Option<Bytes> | null | Uint8Array | Bytes | string) => Observable<Bytes>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
        /** 0xd2bc9897eed08f15/3 */
        taggedTransactionQueue: {
            /**
             * Validate the transaction.
             **/
            validateTransaction: AugmentedCall<
                ApiType,
                (
                    source: TransactionSource | 'InBlock' | 'Local' | 'External' | number | Uint8Array,
                    tx: Extrinsic | IExtrinsic | string | Uint8Array,
                    blockHash: BlockHash | string | Uint8Array,
                ) => Observable<TransactionValidity>
            >;
            /**
             * Generic call
             **/
            [key: string]: DecoratedCallBase<ApiType>;
        };
    } // AugmentedCalls
} // declare module
