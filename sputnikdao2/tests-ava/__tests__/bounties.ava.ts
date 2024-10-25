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
    NEAR,
<<<<<<< HEAD
} from 'near-workspaces';
import {
    initStaking,
    initTestToken,
    initWorkspace,
    Proposal,
=======
} from 'near-workspaces-ava';
import {
    workspace,
    initStaking,
    initTestToken,
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    STORAGE_PER_BYTE,
} from './utils';
import {
    DEADLINE,
    BOND,
    proposeBounty,
    proposeBountyWithNear,
    voteOnBounty,
    claimBounty,
    doneBounty,
    giveupBounty,
    giveupBountyRaw,
    voteApprove,
} from './utils';

<<<<<<< HEAD
const test = initWorkspace();

test('Bounty workflow', async (t) => {
    const { alice, root, dao } = t.context.accounts;
=======
workspace.test('Bounty workflow', async (test, { alice, root, dao }) => {
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    const testToken = await initTestToken(root);
    const proposalId = await proposeBounty(alice, dao, testToken);
    await voteOnBounty(root, dao, proposalId);
    await claimBounty(alice, dao, proposalId);
    const proposal = await dao.view('get_bounty_claims', { account_id: alice });
<<<<<<< HEAD
    t.log('Claims before bounty_done:');
    t.log(await dao.view('get_bounty_claims', { account_id: alice }));
    await doneBounty(alice, alice, dao, proposalId);
    t.log('Claims after bounty_done:');
    t.log(await dao.view('get_bounty_claims', { account_id: alice }));
    t.log('The proposal before act_proposal, voting on the bounty:');
    t.log(await dao.view('get_proposal', { id: proposalId + 1 }));
    await voteOnBounty(root, dao, proposalId + 1);
    t.log('The proposal after act_proposal, voting on the bounty:');
    t.log(await dao.view('get_proposal', { id: proposalId + 1 }));
});

test('Bounty claim', async (t) => {
    const { alice, root, dao } = t.context.accounts;
=======
    test.log('Claims before bounty_done:');
    test.log(await dao.view('get_bounty_claims', { account_id: alice }));
    await doneBounty(alice, alice, dao, proposalId);
    test.log('Claims after bounty_done:');
    test.log(await dao.view('get_bounty_claims', { account_id: alice }));
    test.log('The proposal before act_proposal, voting on the bounty:');
    test.log(await dao.view('get_proposal', { id: proposalId + 1 }));
    await voteOnBounty(root, dao, proposalId + 1);
    test.log('The proposal after act_proposal, voting on the bounty:');
    test.log(await dao.view('get_proposal', { id: proposalId + 1 }));
});

workspace.test('Bounty claim', async (test, { alice, root, dao }) => {
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    const testToken = await initTestToken(root);
    const proposalId = await proposeBounty(alice, dao, testToken);

    //The method could panic if the bounty with given id doesn't exist
    let errorString1 = await captureError(
        async () => await claimBounty(alice, dao, proposalId),
    );
<<<<<<< HEAD
    t.regex(errorString1, /ERR_NO_BOUNTY/);
=======
    test.regex(errorString1, /ERR_NO_BOUNTY/);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

    await voteOnBounty(root, dao, proposalId);

    //Should panic if `attached_deposit`
    //is not equal to the corresponding `bounty_bond`
    //If we attach more than needed:
    let errorString2_1 = await captureError(
        async () =>
            await alice.call(
                dao,
                'bounty_claim',
                {
                    id: proposalId,
                    deadline: DEADLINE,
                },
                {
                    attachedDeposit: new BN(BOND).addn(1),
                },
            ),
    );
<<<<<<< HEAD
    t.regex(errorString2_1, /ERR_BOUNTY_WRONG_BOND/);
=======
    test.regex(errorString2_1, /ERR_BOUNTY_WRONG_BOND/);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    //If we attach less than needed:
    let errorString2_2 = await captureError(
        async () =>
            await alice.call(
                dao,
                'bounty_claim',
                {
                    id: proposalId,
                    deadline: DEADLINE,
                },
                {
                    attachedDeposit: new BN(BOND).subn(1),
                },
            ),
    );
<<<<<<< HEAD
    t.regex(errorString2_2, /ERR_BOUNTY_WRONG_BOND/);
=======
    test.regex(errorString2_2, /ERR_BOUNTY_WRONG_BOND/);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

    //Should panic in case of wrong deadline
    let errorString3 = await captureError(
        async () =>
            await alice.call(
                dao,
                'bounty_claim',
                {
                    id: proposalId,
                    deadline: '1925376849430593582',
                },
                {
                    attachedDeposit: BOND,
                },
            ),
    );
<<<<<<< HEAD
    t.regex(errorString3, /ERR_BOUNTY_WRONG_DEADLINE/);
=======
    test.regex(errorString3, /ERR_BOUNTY_WRONG_DEADLINE/);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

    await claimBounty(alice, dao, proposalId);

    //Should increase number of claims
<<<<<<< HEAD
    t.is(
=======
    test.is(
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        await dao.view('get_bounty_number_of_claims', { id: proposalId }),
        1,
    );

    //Should add this claim to the list of claims, done by this account
    let bounty: any = await dao.view('get_bounty_claims', {
        account_id: alice,
    });
<<<<<<< HEAD
    t.is(bounty[0].bounty_id, 0);
    t.is(bounty[0].deadline, DEADLINE);
    t.is(bounty[0].completed, false);

    await claimBounty(alice, dao, proposalId);
    t.is(
=======
    test.is(bounty[0].bounty_id, 0);
    test.is(bounty[0].deadline, DEADLINE);
    test.is(bounty[0].completed, false);

    await claimBounty(alice, dao, proposalId);
    test.is(
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        await dao.view('get_bounty_number_of_claims', { id: proposalId }),
        2,
    );

    let bounty2: any = await dao.view('get_bounty_claims', {
        account_id: alice,
    });
<<<<<<< HEAD
    t.is(bounty2[1].bounty_id, 0);
    t.is(bounty2[1].deadline, DEADLINE);
    t.is(bounty2[1].completed, false);

    await claimBounty(alice, dao, proposalId);
    t.is(
=======
    test.is(bounty2[1].bounty_id, 0);
    test.is(bounty2[1].deadline, DEADLINE);
    test.is(bounty2[1].completed, false);

    await claimBounty(alice, dao, proposalId);
    test.is(
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        await dao.view('get_bounty_number_of_claims', { id: proposalId }),
        3,
    );

    //Should panic if all bounties are claimed
    let errorString4 = await captureError(
        async () =>
            await alice.call(
                dao,
                'bounty_claim',
                {
                    id: proposalId,
                    deadline: DEADLINE,
                },
                {
                    attachedDeposit: BOND,
                },
            ),
    );
<<<<<<< HEAD
    t.regex(errorString4, /ERR_BOUNTY_ALL_CLAIMED/);
});

test(
    'Bounty done with NEAR token',
    async (t) => {
        const { alice, root, dao } = t.context.accounts;
=======
    test.regex(errorString4, /ERR_BOUNTY_ALL_CLAIMED/);
});

workspace.test(
    'Bounty done with NEAR token',
    async (test, { alice, root, dao }) => {
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        const proposalId = await proposeBountyWithNear(alice, dao);

        await voteOnBounty(root, dao, proposalId);
        await claimBounty(alice, dao, proposalId);

<<<<<<< HEAD
        const bob = await root.createSubAccount('bob');
=======
        const bob = await root.createAccount('bob');
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        //Should panic if the caller is not in the list of claimers
        let errorString1 = await captureError(
            async () => await doneBounty(alice, bob, dao, proposalId),
        );
<<<<<<< HEAD
        t.regex(errorString1, /ERR_NO_BOUNTY_CLAIMS/);
=======
        test.regex(errorString1, /ERR_NO_BOUNTY_CLAIMS/);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

        await claimBounty(bob, dao, proposalId);

        //Should panic if the list of claims for the caller of the method
        //doesn't contain the claim with given ID
        let errorString2 = await captureError(
            async () => await doneBounty(alice, alice, dao, proposalId + 10),
        );
<<<<<<< HEAD
        t.regex(errorString2, /ERR_NO_BOUNTY_CLAIM/);
=======
        test.regex(errorString2, /ERR_NO_BOUNTY_CLAIM/);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

        //`bounty_done` can only be called by the claimer
        let errorString3 = await captureError(
            async () => await doneBounty(alice, bob, dao, proposalId),
        );
<<<<<<< HEAD
        t.regex(errorString3, /ERR_BOUNTY_DONE_MUST_BE_SELF/);
=======
        test.regex(errorString3, /ERR_BOUNTY_DONE_MUST_BE_SELF/);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

        let bounty: any = await dao.view('get_bounty_claims', {
            account_id: alice,
        });
<<<<<<< HEAD
        t.is(bounty[0].completed, false);
=======
        test.is(bounty[0].completed, false);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

        await doneBounty(alice, alice, dao, proposalId);

        //claim is marked as completed
        bounty = await dao.view('get_bounty_claims', { account_id: alice });
<<<<<<< HEAD
        t.is(bounty[0].completed, true);
=======
        test.is(bounty[0].completed, true);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

        let proposal: any = await dao.view('get_proposal', {
            id: proposalId + 1,
        });
<<<<<<< HEAD
        t.is(proposal.status, 'InProgress');
=======
        test.is(proposal.status, 'InProgress');
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

        await voteOnBounty(root, dao, proposalId + 1);

        //proposal is approved
        proposal = await dao.view('get_proposal', { id: proposalId + 1 });
<<<<<<< HEAD
        t.is(proposal.status, 'Approved');
=======
        test.is(proposal.status, 'Approved');
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

        //Should panic if the bounty claim is completed
        let errorString4 = await captureError(
            async () => await doneBounty(alice, alice, dao, proposalId),
        );
<<<<<<< HEAD
        t.regex(errorString4, /ERR_NO_BOUNTY_CLAIMS/);
    },
);

test('Bounty giveup', async (t) => {
    const { alice, root, dao } = t.context.accounts;
=======
        test.regex(errorString4, /ERR_NO_BOUNTY_CLAIMS/);
    },
);

workspace.test('Bounty giveup', async (test, { alice, root, dao }) => {
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    const testToken = await initTestToken(root);
    const proposalId = await proposeBounty(alice, dao, testToken);
    await voteOnBounty(root, dao, proposalId);
    await claimBounty(alice, dao, proposalId);

    //Should panic if the caller is not in the list of claimers
<<<<<<< HEAD
    const bob = await root.createSubAccount('bob');
    let errorString = await captureError(
        async () => await giveupBounty(bob, dao, proposalId),
    );
    t.regex(errorString, /ERR_NO_BOUNTY_CLAIMS/);
=======
    const bob = await root.createAccount('bob');
    let errorString = await captureError(
        async () => await giveupBounty(bob, dao, proposalId),
    );
    test.regex(errorString, /ERR_NO_BOUNTY_CLAIMS/);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

    //Should panic if the list of claims for the caller of the method
    //doesn't contain the claim with given ID
    errorString = await captureError(
        async () => await giveupBounty(alice, dao, proposalId + 10),
    );
<<<<<<< HEAD
    t.regex(errorString, /ERR_NO_BOUNTY_CLAIM/);
=======
    test.regex(errorString, /ERR_NO_BOUNTY_CLAIM/);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c

    //If within forgiveness period, `bounty_bond` should be returned ???
    const balance1: NEAR = (await alice.balance()).total;
    const result = await giveupBountyRaw(alice, dao, proposalId);
    const balance2: NEAR = (await alice.balance()).total;
<<<<<<< HEAD
    t.is(
=======
    test.is(
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        Number(balance2.add(result.gas_burnt).toHuman().slice(0, -1)).toFixed(
            1,
        ),
        Number(balance1.add(ONE_NEAR).toHuman().slice(0, -1)).toFixed(1),
    );
<<<<<<< HEAD
    t.not(balance2, balance1);

    //If within forgiveness period,
    //claim should be removed from the list of claims, done by this account
    t.deepEqual(
=======
    test.not(balance2, balance1);

    //If within forgiveness period,
    //claim should be removed from the list of claims, done by this account
    test.deepEqual(
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        await dao.view('get_bounty_claims', { account_id: alice }),
        [],
    );
});

<<<<<<< HEAD
test('Bounty ft done', async (t) => {
    const { alice, root, dao } = t.context.accounts;
=======
workspace.test('Bounty ft done', async (test, { alice, root, dao }) => {
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    const testToken = await initTestToken(root);
    await dao.call(
        testToken,
        'mint',
        {
            account_id: dao,
            amount: '1000000000',
        },
        {
            gas: tGas(50),
        },
    );
    await alice.call(
        testToken,
        'storage_deposit',
        {
            account_id: alice.accountId,
            registration_only: true,
        },
        {
            attachedDeposit: toYocto('90'),
        },
    );
    const bounty = {
        description: 'test_bounties',
        token: testToken.accountId,
        amount: '10',
        times: 3,
        max_deadline: DEADLINE,
    };
    let proposalId: number = await alice.call(
        dao,
        'add_proposal',
        {
            proposal: {
                description: 'add_new_bounty',
                kind: {
                    AddBounty: {
                        bounty,
                    },
                },
            },
        },
        {
            attachedDeposit: toYocto('1'),
        },
    );
    await voteApprove(root, dao, proposalId);
<<<<<<< HEAD
    let { status }: Proposal = await dao.view('get_proposal', { id: proposalId });
    t.is(status, 'Approved');
=======
    let { status } = await dao.view('get_proposal', { id: proposalId });
    test.is(status, 'Approved');
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    const bountyId = 0; // first bounty
    await claimBounty(alice, dao, bountyId);
    await alice.call(
        dao,
        'bounty_done',
        {
            id: bountyId,
            account_id: alice.accountId,
            description: 'This bounty is done',
        },
        {
            attachedDeposit: toYocto('1'),
        },
    );

    await voteApprove(root, dao, proposalId + 1);
<<<<<<< HEAD
    ({ status } = await dao.view('get_proposal', { id: proposalId }) as Proposal );
    t.is(status, 'Approved');
});

test(
    'Callback for BountyDone with NEAR token',
    async (t) => {
        const { alice, root, dao } = t.context.accounts;
=======
    ({ status } = await dao.view('get_proposal', { id: proposalId }));
    test.is(status, 'Approved');
});

workspace.test(
    'Callback for BountyDone with NEAR token',
    async (test, { alice, root, dao }) => {
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        //During the callback the number bounty_claims_count should decrease
        const proposalId = await proposeBountyWithNear(alice, dao);
        await voteOnBounty(root, dao, proposalId);
        await claimBounty(alice, dao, proposalId);
        await doneBounty(alice, alice, dao, proposalId);
        //Before the bounty is done there is 1 claim
<<<<<<< HEAD
        t.is(await dao.view('get_bounty_number_of_claims', { id: 0 }), 1);
=======
        test.is(await dao.view('get_bounty_number_of_claims', { id: 0 }), 1);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        const balanceBefore: NEAR = (await alice.balance()).total;
        //During the callback this number is decreased
        await voteOnBounty(root, dao, proposalId + 1);
        const balanceAfter: NEAR = (await alice.balance()).total;
<<<<<<< HEAD
        t.is(await dao.view('get_bounty_number_of_claims', { id: 0 }), 0);
        t.assert(balanceBefore.lt(balanceAfter));
    },
);

test(
    'Callback for BountyDone ft token fail',
    async (t) => {
        const { alice, root, dao } = t.context.accounts;
=======
        test.is(await dao.view('get_bounty_number_of_claims', { id: 0 }), 0);
        test.assert(balanceBefore.lt(balanceAfter));
    },
);

workspace.test(
    'Callback for BountyDone ft token fail',
    async (test, { alice, root, dao }) => {
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        //Test the callback with Failed proposal status
        const testTokenFail = await initTestToken(root);
        const proposalIdFail = await proposeBounty(alice, dao, testTokenFail);
        await dao.call(
            testTokenFail,
            'mint',
            {
                account_id: dao,
                amount: '1000000000',
            },
            {
                gas: tGas(50),
            },
        );
        await alice.call(
            testTokenFail,
            'storage_deposit',
            {
                account_id: alice.accountId,
                registration_only: true,
            },
            {
                attachedDeposit: toYocto('90'),
            },
        );
        await voteOnBounty(root, dao, proposalIdFail);
        await claimBounty(alice, dao, proposalIdFail);
        await doneBounty(alice, alice, dao, proposalIdFail);
        await voteOnBounty(root, dao, proposalIdFail + 1);
        //Proposal should be Failed
<<<<<<< HEAD
        let { status }: Proposal = await dao.view('get_proposal', {
            id: proposalIdFail + 1,
        });
        t.is(status, 'Failed');
    },
);

test(
    'Callback for BountyDone ft token',
    async (t) => {
        const { alice, root, dao } = t.context.accounts;
=======
        let { status } = await dao.view('get_proposal', {
            id: proposalIdFail + 1,
        });
        test.is(status, 'Failed');
    },
);

workspace.test(
    'Callback for BountyDone ft token',
    async (test, { alice, root, dao }) => {
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        //Test correct callback
        const testToken = await initTestToken(root);
        await dao.call(
            testToken,
            'mint',
            {
                account_id: dao,
                amount: '1000000000',
            },
            {
                gas: tGas(50),
            },
        );
        await alice.call(
            testToken,
            'storage_deposit',
            {
                account_id: alice.accountId,
                registration_only: true,
            },
            {
                attachedDeposit: toYocto('90'),
            },
        );
        const bounty = {
            description: 'test_bounties',
            token: testToken.accountId,
            amount: '10',
            times: 3,
            max_deadline: DEADLINE,
        };
        let proposalId: number = await alice.call(
            dao,
            'add_proposal',
            {
                proposal: {
                    description: 'add_new_bounty',
                    kind: {
                        AddBounty: {
                            bounty,
                        },
                    },
                },
            },
            {
                attachedDeposit: toYocto('1'),
            },
        );
        await voteOnBounty(root, dao, proposalId);
        await claimBounty(alice, dao, proposalId);
        await doneBounty(alice, alice, dao, proposalId);
        //Before the bounty is done there is 1 claim
<<<<<<< HEAD
        t.is(await dao.view('get_bounty_number_of_claims', { id: 0 }), 1);
=======
        test.is(await dao.view('get_bounty_number_of_claims', { id: 0 }), 1);
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
        const balanceBefore: NEAR = (await alice.balance()).total;
        //During the callback this number is decreased
        await voteOnBounty(root, dao, proposalId + 1);

        //Proposal should be approved
<<<<<<< HEAD
        let { status }: Proposal = await dao.view('get_proposal', { id: proposalId + 1 });
        t.is(status, 'Approved');

        //During the callback the number bounty_claims_count should decrease
        const balanceAfter: NEAR = (await alice.balance()).total;
        t.is(await dao.view('get_bounty_number_of_claims', { id: 0 }), 0);
        t.assert(balanceBefore.lt(balanceAfter));
=======
        let { status } = await dao.view('get_proposal', { id: proposalId + 1 });
        test.is(status, 'Approved');

        //During the callback the number bounty_claims_count should decrease
        const balanceAfter: NEAR = (await alice.balance()).total;
        test.is(await dao.view('get_bounty_number_of_claims', { id: 0 }), 0);
        test.assert(balanceBefore.lt(balanceAfter));
>>>>>>> 4c04023d81c526af92d771dc71a1f2216de3f45c
    },
);
