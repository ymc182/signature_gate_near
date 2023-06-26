import { Worker, NearAccount } from "near-workspaces";
import anyTest, { TestFn } from "ava";

const test = anyTest as TestFn<{
	worker: Worker;
	accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
	// Init the worker and start a Sandbox server
	const worker = await Worker.init();

	// Deploy contract
	const root = worker.rootAccount;
	const userAccount = await root.createSubAccount("user-account");
	const contract = await root.createSubAccount("test-account");
	// Get wasm file path from package.json test script in folder above
	await contract.deploy(process.argv[2]);
	console.log("Contract deployed to:", contract.accountId);
	// Save state for test runs, it is unique for each test
	t.context.worker = worker;
	t.context.accounts = { root, contract, userAccount };
});

test.afterEach.always(async (t) => {
	// Stop Sandbox server
	await t.context.worker.tearDown().catch((error) => {
		console.log("Failed to stop the Sandbox:", error);
	});
});

test("Should verify the correct msg", async (t) => {
	const { contract, root, userAccount } = t.context.accounts;

	const rootKP = await root.getKey();

	const rootPubKey = (await root.getKey())?.getPublicKey().toString().split(":")[1];
	const userPubKey = (await userAccount.getKey())?.getPublicKey().toString().split(":")[1];

	await root.call(contract.accountId, "set_verifier_pub_key", {
		verifier_pub_key: rootPubKey,
	});

	// mimic the server side
	// on server side, no transaction is needed
	const message = {
		expiration: 100000000000000,
		reward: 10000000000,
		pub_key: userPubKey,
	};

	const fakeMessage = {
		expiration: 100000000000000,
		reward: 1000000000000,
		pub_key: userPubKey,
	};

	const messageStr = JSON.stringify(message);
	const fakeMessageStr = JSON.stringify(fakeMessage);
	const signature = rootKP?.sign(Buffer.from(messageStr));
	const sign_hex = Buffer.from(signature?.signature!).toString("hex");

	// mimic the client side
	const result = await userAccount.call(contract.accountId, "verify_signature", {
		signature_hex: sign_hex,
		declared_message: messageStr,
	});

	const falseResult = await userAccount.call(contract.accountId, "verify_signature", {
		signature_hex: sign_hex,
		declared_message: fakeMessageStr,
	});

	t.deepEqual(result, true);
	t.deepEqual(falseResult, false);
});
