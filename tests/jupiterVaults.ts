import * as anchor from '@coral-xyz/anchor';
import {ConfirmOptions, Keypair} from '@solana/web3.js';
import { assert } from 'chai';
import {
	getVaultAddressSync,
	JupiterVaults,
	encodeName,
	VaultParams,
	getTokenVaultAddressSync,
} from "../ts/sdk";
import {BN} from "@coral-xyz/anchor";
import {mockUSDCMint} from "./testHelpers";

describe('jupiterVaults', () => {
	const opts: ConfirmOptions = {
		preflightCommitment: 'confirmed',
		skipPreflight: false,
		commitment: 'confirmed',
	};

	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.local(undefined, opts);
	anchor.setProvider(provider);
	const program = anchor.workspace.JupiterVaults as anchor.Program<JupiterVaults>;

	// const initialSolPerpPrice = 100;

	const _manager = provider.publicKey;
	const protocol = provider.publicKey;
	let mint: Keypair;

	const name = 'Test Vault';
	const vaultKey = getVaultAddressSync(program.programId, encodeName(name));
	const vaultAta = getTokenVaultAddressSync(
		program.programId,
		vaultKey,
	);

	before(async () => {
		mint = await mockUSDCMint(provider);
		// const _solPerpOracle = await mockOracle(initialSolPerpPrice, undefined, undefined);
	});

	it('Initialize Vault', async () => {
		const config: VaultParams = {
			name: encodeName(name),
			redeemPeriod: new BN(0),
			maxTokens: new BN(0),
			managementFee: new BN(0),
			minDepositAmount: new BN(0),
			profitShare: 0,
			hurdleRate: 0,
			permissioned: false,
			protocol,
			protocolFee: new BN(0),
			protocolProfitShare: 0,
		};
		const accounts = {
			vault: vaultKey,
			tokenAccount: vaultAta,
			mint: mint.publicKey,
		};
		// @ts-ignore
		await program.methods.initializeVault(config)
			.accounts(accounts)
			.rpc();
		const acct = await program.account.vault.fetch(vaultKey);
		assert(!!acct);
	});

});
