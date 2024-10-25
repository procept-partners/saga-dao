import {
<<<<<<< HEAD
=======
    Workspace,
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    BN,
    NearAccount,
    captureError,
    toYocto,
    tGas,
    ONE_NEAR,
<<<<<<< HEAD
} from 'near-workspaces';
import {
    initWorkspace,
=======
} from 'near-workspaces-ava';
import {
    workspace,
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    initStaking,
    initTestToken,
    setStakingId,
    registerAndDelegate,
    STORAGE_PER_BYTE,
<<<<<<< HEAD
    deployAndInit,
=======
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
} from './utils';
import {
    DEADLINE,
    BOND,
    proposeBounty,
    voteOnBounty,
    claimBounty,
} from './utils';
import * as fs from 'fs';

<<<<<<< HEAD
const test = initWorkspace();

test('View method version', async (t) => {
    const { alice, root, dao } = t.context.accounts;
    t.log('Version:');
    t.log(await dao.view('version'));
    t.is(await dao.view('version'), '2.3.1');
});

test('View method get_config', async (t) => {
    const { root } = t.context.accounts;
=======
workspace.test('View method version', async (test, { alice, root, dao }) => {
    test.log('Version:');
    test.log(await dao.view('version'));
    test.is(await dao.view('version'), '2.0.0');
});

workspace.test('View method get_config', async (test, { root }) => {
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    const config = {
        name: 'sputnikda2',
        purpose: 'testing get_config',
        metadata: '',
    };
    const policy = [root.accountId];

<<<<<<< HEAD
    const bob = await deployAndInit({
        root,
        subContractId: 'bob',
        code: '../res/sputnikdao2.wasm',
        init: {
            methodName: 'new',
            args: { config, policy }
        },
        initialBalance: toYocto('200'),
    });
    t.deepEqual(await bob.view('get_config'), config);
});

test('View method get_policy', async (t) => {
    const { root } = t.context.accounts;
=======
    const bob = await root.createAndDeploy('bob', '../res/sputnikdao2.wasm', {
        method: 'new',
        args: { config, policy },
        initialBalance: toYocto('200'),
    });
    test.deepEqual(await bob.view('get_config'), config);
});

workspace.test('View method get_policy', async (test, { root }) => {
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    const config = {
        name: 'sputnikda2',
        purpose: 'testing get_policy',
        metadata: '',
    };
    const versionedPolicy = [root.accountId];

<<<<<<< HEAD
    const bob = await deployAndInit({
        root,
        subContractId: 'bob',
        code: '../res/sputnikdao2.wasm',
        init: {
            methodName: 'new',
            args: { config, policy: versionedPolicy },
        },
=======
    const bob = await root.createAndDeploy('bob', '../res/sputnikdao2.wasm', {
        method: 'new',
        args: { config, policy: versionedPolicy },
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        initialBalance: toYocto('200'),
    });
    const policy = {
        roles: [
            {
                name: 'all',
                kind: 'Everyone',
                permissions: ['*:AddProposal'],
                vote_policy: {},
            },
            {
                name: 'council',
                kind: {
                    Group: [root.accountId],
                },
                permissions: [
                    '*:Finalize',
                    '*:AddProposal',
                    '*:VoteApprove',
                    '*:VoteReject',
                    '*:VoteRemove',
                ],
                vote_policy: {},
            },
        ],
        default_vote_policy: {
            weight_kind: 'RoleWeight',
            quorum: '0',
            threshold: [1, 2],
        },
        proposal_bond: '1000000000000000000000000',
        proposal_period: '604800000000000',
        bounty_bond: '1000000000000000000000000',
        bounty_forgiveness_period: '86400000000000',
    };
<<<<<<< HEAD
    t.deepEqual(await bob.view('get_policy'), policy);
});

test(
    'View method get_staking_contract',
    async (t) => {
        const { alice, root, dao } = t.context.accounts;
        t.is(await dao.view('get_staking_contract'), '');
=======
    test.deepEqual(await bob.view('get_policy'), policy);
});

workspace.test(
    'View method get_staking_contract',
    async (test, { alice, root, dao }) => {
        test.is(await dao.view('get_staking_contract'), '');
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

        //To set the staking_id
        const testToken = await initTestToken(root);
        const staking = await initStaking(root, dao, testToken);
        await setStakingId(root, dao, staking);

<<<<<<< HEAD
        t.is(await dao.view('get_staking_contract'), staking.accountId);
    },
);

test('View has_blob', async (t) => {
    const { alice, root, dao } = t.context.accounts;
=======
        test.is(await dao.view('get_staking_contract'), staking.accountId);
    },
);

workspace.test('View has_blob', async (test, { alice, root, dao }) => {
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    const DAO_WASM_BYTES: Uint8Array = fs.readFileSync(
        '../res/sputnikdao2.wasm',
    );
    const hash: String = await root.call(dao, 'store_blob', DAO_WASM_BYTES, {
        attachedDeposit: toYocto('200'),
        gas: tGas(300),
    });

<<<<<<< HEAD
    t.true(await dao.view('has_blob', { hash: hash }));
    await root.call(dao, 'remove_blob', {
        hash: hash,
    });
    t.false(await dao.view('has_blob', { hash: hash }));
});

test(
    'View get_locked_storage_amount',
    async (t) => {
        const { alice, root, dao } = t.context.accounts;
        const beforeProposal = new BN(
            await dao.view('get_locked_storage_amount'),
        );
        t.log('Locked amount: ' + beforeProposal);
=======
    test.true(await dao.view('has_blob', { hash: hash }));
    await root.call(dao, 'remove_blob', {
        hash: hash,
    });
    test.false(await dao.view('has_blob', { hash: hash }));
});

workspace.test(
    'View get_locked_storage_amount',
    async (test, { alice, root, dao }) => {
        const beforeProposal = new BN(
            await dao.view('get_locked_storage_amount'),
        );
        test.log('Locked amount: ' + beforeProposal);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        await root.call(
            dao,
            'add_proposal',
            {
                proposal: {
                    description: 'adding some bytes',
                    kind: 'Vote',
                },
            },
            {
                attachedDeposit: toYocto('1'),
            },
        );
        const afterProposal = new BN(
            await dao.view('get_locked_storage_amount'),
        );
<<<<<<< HEAD
        t.assert(beforeProposal.lt(afterProposal));
    },
);

test(
    'View get_available_amount',
    async (t) => {
        const { alice, root, dao } = t.context.accounts;
        const beforeProposal = new BN(await dao.view('get_available_amount'));
        t.log('Available amount: ' + beforeProposal);
=======
        test.assert(beforeProposal.lt(afterProposal));
    },
);

workspace.test(
    'View get_available_amount',
    async (test, { alice, root, dao }) => {
        const beforeProposal = new BN(await dao.view('get_available_amount'));
        test.log('Available amount: ' + beforeProposal);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        await root.call(
            dao,
            'add_proposal',
            {
                proposal: {
                    description: 'adding some bytes',
                    kind: 'Vote',
                },
            },
            {
                attachedDeposit: toYocto('1'),
            },
        );
        const afterProposal = new BN(await dao.view('get_available_amount'));
<<<<<<< HEAD
        t.assert(beforeProposal.gt(afterProposal));
    },
);

test(
    'View methods for delegation',
    async (t) => {
        const { alice, root, dao } = t.context.accounts;
        const testToken = await initTestToken(root);
        const staking = await initStaking(root, dao, testToken);
        const randomAmount = new BN('10087687667869');
        const bob = await root.createSubAccount('bob');
=======
        test.assert(beforeProposal.gt(afterProposal));
    },
);

workspace.test(
    'View methods for delegation',
    async (test, { alice, root, dao }) => {
        const testToken = await initTestToken(root);
        const staking = await initStaking(root, dao, testToken);
        const randomAmount = new BN('10087687667869');
        const bob = await root.createAccount('bob');
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

        await setStakingId(root, dao, staking);

        let result = await registerAndDelegate(
            dao,
            staking,
            alice,
            randomAmount,
        );
        result = await registerAndDelegate(
            dao,
            staking,
            bob,
            randomAmount.muln(2),
        );

        //Test delegation_balance_of
<<<<<<< HEAD
        t.deepEqual(
=======
        test.deepEqual(
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
            new BN(
                await dao.view('delegation_balance_of', { account_id: alice }),
            ),
            randomAmount,
        );
<<<<<<< HEAD
        t.deepEqual(
=======
        test.deepEqual(
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
            new BN(
                await dao.view('delegation_balance_of', { account_id: bob }),
            ),
            randomAmount.muln(2),
        );

        //Test delegation_total_supply
<<<<<<< HEAD
        t.deepEqual(
=======
        test.deepEqual(
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
            new BN(await dao.view('delegation_total_supply')),
            randomAmount.muln(3),
        );

        //Test delegation_balance_ratio
<<<<<<< HEAD
        t.deepEqual(
=======
        test.deepEqual(
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
            await dao.view('delegation_balance_ratio', { account_id: alice }),
            [
                await dao.view('delegation_balance_of', { account_id: alice }),
                await dao.view('delegation_total_supply'),
            ],
        );
    },
);

<<<<<<< HEAD
test(
    'View methods for proposals',
    async (t) => {
        const { alice, root, dao } = t.context.accounts;
        //Test get_last_proposal_id
        t.is(await dao.view('get_last_proposal_id'), 0);

        //Test get_proposals
        t.deepEqual(
=======
workspace.test(
    'View methods for proposals',
    async (test, { alice, root, dao }) => {
        //Test get_last_proposal_id
        test.is(await dao.view('get_last_proposal_id'), 0);

        //Test get_proposals
        test.deepEqual(
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
            await dao.view('get_proposals', { from_index: 0, limit: 100 }),
            [],
        );

        const config = {
            name: 'sputnikdao2',
            purpose: 'testing_view_methods',
            metadata: '',
        };
        await alice.call(
            dao,
            'add_proposal',
            {
                proposal: {
                    description: 'rename the dao',
                    kind: {
                        ChangeConfig: {
                            config,
                        },
                    },
                },
            },
            { attachedDeposit: toYocto('1') },
        );

        const realProposalAlice = {
            id: 0,
            proposer: alice.accountId,
            description: 'rename the dao',
            kind: { ChangeConfig: { config } },
            status: 'InProgress',
            vote_counts: {},
            votes: {},
        };

        const proposalAlice: any = await dao.view('get_proposal', { id: 0 });

        //Test get_proposal
<<<<<<< HEAD
        t.is(proposalAlice.proposer, realProposalAlice.proposer);
        t.is(proposalAlice.description, realProposalAlice.description);
        t.is(proposalAlice.status, realProposalAlice.status);
        t.deepEqual(
            proposalAlice.vote_counts,
            realProposalAlice.vote_counts,
        );
        t.deepEqual(proposalAlice.votes, realProposalAlice.votes);
        t.deepEqual(proposalAlice.kind, realProposalAlice.kind);

        //Test get_last_proposal_id
        t.deepEqual(await dao.view('get_last_proposal_id'), 1);
=======
        test.is(proposalAlice.proposer, realProposalAlice.proposer);
        test.is(proposalAlice.description, realProposalAlice.description);
        test.is(proposalAlice.status, realProposalAlice.status);
        test.deepEqual(
            proposalAlice.vote_counts,
            realProposalAlice.vote_counts,
        );
        test.deepEqual(proposalAlice.votes, realProposalAlice.votes);
        test.deepEqual(proposalAlice.kind, realProposalAlice.kind);

        //Test get_last_proposal_id
        test.deepEqual(await dao.view('get_last_proposal_id'), 1);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

        //Test get_proposals
        const proposals: any = await dao.view('get_proposals', {
            from_index: 0,
            limit: 100,
        });
<<<<<<< HEAD
        t.is(proposals[0].proposer, realProposalAlice.proposer);
        t.is(proposals[0].description, realProposalAlice.description);
        t.is(proposals[0].status, realProposalAlice.status);
        t.deepEqual(proposals[0].vote_counts, realProposalAlice.vote_counts);
        t.deepEqual(proposals[0].votes, realProposalAlice.votes);
        t.deepEqual(proposals[0].kind, realProposalAlice.kind);
=======
        test.is(proposals[0].proposer, realProposalAlice.proposer);
        test.is(proposals[0].description, realProposalAlice.description);
        test.is(proposals[0].status, realProposalAlice.status);
        test.deepEqual(proposals[0].vote_counts, realProposalAlice.vote_counts);
        test.deepEqual(proposals[0].votes, realProposalAlice.votes);
        test.deepEqual(proposals[0].kind, realProposalAlice.kind);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

        //Should panic if the proposal with the given id doesn't exist
        const errorString = await captureError(
            async () => await dao.view('get_proposal', { id: 10 }),
        );
<<<<<<< HEAD
        t.regex(errorString, /ERR_NO_PROPOSAL/);
    },
);

test(
    'View methods for bounties',
    async (t) => {
        const { alice, root, dao } = t.context.accounts;
        //Test get_last_bounty_id
        t.is(await dao.view('get_last_bounty_id'), 0);
        //Test get_bounties
        t.deepEqual(
=======
        test.regex(errorString, /ERR_NO_PROPOSAL/);
    },
);

workspace.test(
    'View methods for bounties',
    async (test, { alice, root, dao }) => {
        //Test get_last_bounty_id
        test.is(await dao.view('get_last_bounty_id'), 0);
        //Test get_bounties
        test.deepEqual(
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
            await dao.view('get_bounties', { from_index: 0, limit: 100 }),
            [],
        );

        const testToken = await initTestToken(root);
        const proposalId = await proposeBounty(alice, dao, testToken);
        const bounty = {
            id: 0,
            description: 'test_bounties',
            token: testToken.accountId,
            amount: '19000000000000000000000000',
            times: 3,
            max_deadline: DEADLINE,
        };
        await voteOnBounty(root, dao, proposalId);

        //Test get_last_bounty_id
<<<<<<< HEAD
        t.is(await dao.view('get_last_bounty_id'), 1);
        //Test get_bounties
        t.deepEqual(
=======
        test.is(await dao.view('get_last_bounty_id'), 1);
        //Test get_bounties
        test.deepEqual(
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
            await dao.view('get_bounties', { from_index: 0, limit: 100 }),
            [bounty],
        );
        //Test get_bounty
<<<<<<< HEAD
        t.deepEqual(await dao.view('get_bounty', { id: 0 }), bounty);
=======
        test.deepEqual(await dao.view('get_bounty', { id: 0 }), bounty);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

        await claimBounty(alice, dao, proposalId);

        //Test get_bounty_number_of_claims
<<<<<<< HEAD
        t.is(await dao.view('get_bounty_number_of_claims', { id: 0 }), 1);
=======
        test.is(await dao.view('get_bounty_number_of_claims', { id: 0 }), 1);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        //Test get_bounty_claims
        const realClaim = {
            bounty_id: 0,
            deadline: DEADLINE,
            completed: false,
        };
        const claims: any = await dao.view('get_bounty_claims', {
            account_id: alice.accountId,
        });
<<<<<<< HEAD
        t.is(claims[0].bounty_id, realClaim.bounty_id);
        t.is(claims[0].deadline, realClaim.deadline);
        t.is(claims[0].completed, realClaim.completed);
=======
        test.is(claims[0].bounty_id, realClaim.bounty_id);
        test.is(claims[0].deadline, realClaim.deadline);
        test.is(claims[0].completed, realClaim.completed);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

        //Should panic if the bounty with the given id doesn't exist
        const errorString = await captureError(
            async () => await dao.view('get_bounty', { id: 10 }),
        );
<<<<<<< HEAD
        t.regex(errorString, /ERR_NO_BOUNTY/);
=======
        test.regex(errorString, /ERR_NO_BOUNTY/);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    },
);
