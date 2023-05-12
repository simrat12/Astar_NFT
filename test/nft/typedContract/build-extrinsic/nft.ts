/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/nft';
import type BN from 'bn.js';
import type { ApiPromise } from '@polkadot/api';



export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;

	constructor(
		nativeContract : ContractPromise,
		apiPromise: ApiPromise,
	) {
		this.__nativeContract = nativeContract;
		this.__apiPromise = apiPromise;
	}
	/**
	 * ownerOf
	 *
	 * @param { ArgumentTypes.Id } id,
	*/
	"ownerOf" (
		id: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::ownerOf", [id], __options);
	}

	/**
	 * collectionId
	 *
	*/
	"collectionId" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::collectionId", [], __options);
	}

	/**
	 * totalSupply
	 *
	*/
	"totalSupply" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::totalSupply", [], __options);
	}

	/**
	 * approve
	 *
	 * @param { ArgumentTypes.AccountId } operator,
	 * @param { ArgumentTypes.Id | null } id,
	 * @param { boolean } approved,
	*/
	"approve" (
		operator: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id | null,
		approved: boolean,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::approve", [operator, id, approved], __options);
	}

	/**
	 * balanceOf
	 *
	 * @param { ArgumentTypes.AccountId } owner,
	*/
	"balanceOf" (
		owner: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::balanceOf", [owner], __options);
	}

	/**
	 * transfer
	 *
	 * @param { ArgumentTypes.AccountId } to,
	 * @param { ArgumentTypes.Id } id,
	 * @param { Array<(number | string | BN)> } data,
	*/
	"transfer" (
		to: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id,
		data: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::transfer", [to, id, data], __options);
	}

	/**
	 * allowance
	 *
	 * @param { ArgumentTypes.AccountId } owner,
	 * @param { ArgumentTypes.AccountId } operator,
	 * @param { ArgumentTypes.Id | null } id,
	*/
	"allowance" (
		owner: ArgumentTypes.AccountId,
		operator: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id | null,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::allowance", [owner, operator, id], __options);
	}

	/**
	 * tokenByIndex
	 *
	 * @param { (string | number | BN) } index,
	*/
	"tokenByIndex" (
		index: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Enumerable::tokenByIndex", [index], __options);
	}

	/**
	 * ownersTokenByIndex
	 *
	 * @param { ArgumentTypes.AccountId } owner,
	 * @param { (string | number | BN) } index,
	*/
	"ownersTokenByIndex" (
		owner: ArgumentTypes.AccountId,
		index: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Enumerable::ownersTokenByIndex", [owner, index], __options);
	}

	/**
	 * getAttribute
	 *
	 * @param { ArgumentTypes.Id } id,
	 * @param { Array<(number | string | BN)> } key,
	*/
	"getAttribute" (
		id: ArgumentTypes.Id,
		key: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Metadata::getAttribute", [id, key], __options);
	}

	/**
	 * owner
	 *
	*/
	"owner" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "ownable::owner", [], __options);
	}

	/**
	 * transferOwnership
	 *
	 * @param { ArgumentTypes.AccountId } newOwner,
	*/
	"transferOwnership" (
		newOwner: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "ownable::transferOwnership", [newOwner], __options);
	}

	/**
	 * renounceOwnership
	 *
	*/
	"renounceOwnership" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "ownable::renounceOwnership", [], __options);
	}

	/**
	 * mintNext
	 *
	*/
	"mintNext" (
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "payableMint::mintNext", [], __options);
	}

	/**
	 * tokenUri
	 *
	 * @param { (number | string | BN) } tokenId,
	*/
	"tokenUri" (
		tokenId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "payableMint::tokenUri", [tokenId], __options);
	}

	/**
	 * withdraw
	 *
	*/
	"withdraw" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "payableMint::withdraw", [], __options);
	}

	/**
	 * price
	 *
	*/
	"price" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "payableMint::price", [], __options);
	}

	/**
	 * getMaxMintAmount
	 *
	*/
	"getMaxMintAmount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "payableMint::getMaxMintAmount", [], __options);
	}

	/**
	 * mint
	 *
	 * @param { ArgumentTypes.AccountId } to,
	 * @param { (number | string | BN) } mintAmount,
	*/
	"mint" (
		to: ArgumentTypes.AccountId,
		mintAmount: (number | string | BN),
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "payableMint::mint", [to, mintAmount], __options);
	}

	/**
	 * setBaseUri
	 *
	 * @param { string } uri,
	*/
	"setBaseUri" (
		uri: string,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "payableMint::setBaseUri", [uri], __options);
	}

	/**
	 * setMaxMintAmount
	 *
	 * @param { (number | string | BN) } maxAmount,
	*/
	"setMaxMintAmount" (
		maxAmount: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "payableMint::setMaxMintAmount", [maxAmount], __options);
	}

	/**
	 * maxSupply
	 *
	*/
	"maxSupply" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "payableMint::maxSupply", [], __options);
	}

	/**
	 * attack
	 *
	 * @param { (number | string | BN) } nft1Id,
	 * @param { (number | string | BN) } nft2Id,
	*/
	"attack" (
		nft1Id: (number | string | BN),
		nft2Id: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "payableMint::attack", [nft1Id, nft2Id], __options);
	}

	/**
	 * hp
	 *
	 * @param { (number | string | BN) } tokenId,
	*/
	"hp" (
		tokenId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "payableMint::hp", [tokenId], __options);
	}

}