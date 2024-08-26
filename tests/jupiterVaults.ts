import * as anchor from '@coral-xyz/anchor';
import {
	mockOracle,
	mockUSDCMint,
} from './testHelpers';
import { ConfirmOptions } from '@solana/web3.js';
import { assert } from 'chai';

describe('driftVaults', () => {
	const opts: ConfirmOptions = {
		preflightCommitment: 'confirmed',
		skipPreflight: false,
		commitment: 'confirmed',
	};

	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.local(undefined, opts);
	anchor.setProvider(provider);
	// const connection = provider.connection;
	// const program = anchor.workspace.DriftVaults as anchor.Program<DriftVaults>;

	const initialSolPerpPrice = 100;

	before(async () => {
		const _usdcMint = await mockUSDCMint(provider);
		const _solPerpOracle = await mockOracle(initialSolPerpPrice, undefined, undefined);
	});

	after(async () => {

	});

	it('Initialize Vault', async () => {
		assert(true);
	});

});
