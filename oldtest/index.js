orchestrator.registerScenario('create goal test', async (s, t) => {
  // the 'true' is for 'start', which means boot the Conductors
  const { alice } = await s.players({ alice: projectsConfig }, true)
  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const addr = await alice.call('projects', 'acorn_projects', 'create_goal', {
    goal: {
      content: 'sample content',
      user_hash: alice.info('projects').agentAddress,
      timestamp_created: Date.now(),
      hierarchy: 'Branch',
      status: 'Uncertain',
      description: ''
    },
    maybe_goal_edge_input: null
  })

  // Wait for all network activity to
  await s.consistency()
  const result1 = await alice.call(
    'projects',
    'acorn_projects',
    'add_member_of_goal',
    {
      goal_member: {
        goal_address: addr.Ok.goal.address,
        agent_address: alice.info('projects').agentAddress,
        unix_timestamp: Date.now()
      }
    }
  )

  await s.consistency()
  const result2 = await alice.call(
    'projects',
    'acorn_projects',
    'archive_member_of_goal',
    {
      address: result1.Ok.address
    }
  )

  await s.consistency()

  // check for equality of the actual and expected results
  t.deepEqual(result1.Ok.address, result2.Ok)
})

orchestrator.registerScenario(
  'goal create/update/history/archive test',
  async (s, t) => {
    // the 'true' is for 'start', which means boot the Conductors
    const { alice } = await s.players({ alice: projectsConfig }, true)
    // Make a call to a Zome function
    // indicating the function, and passing it an input
    const addr = await alice.call('projects', 'acorn_projects', 'create_goal', {
      goal: {
        content: 'sample content',
        user_hash: alice.info('projects').agentAddress,
        timestamp_created: Date.now(),
        hierarchy: 'Branch',
        status: 'Uncertain',
        description: ''
      },
      maybe_goal_edge_input: null
    })
    await s.consistency()
    const addr2 = await alice.call(
      'projects',
      'acorn_projects',
      'create_goal',
      {
        goal: {
          content: 'sample content',
          user_hash: alice.info('projects').agentAddress,
          timestamp_created: Date.now(),
          hierarchy: 'Branch',
          status: 'Uncertain',
          description: ''
        },
        maybe_goal_edge_input: null
      }
    )
    await s.consistency()
    await alice.call('projects', 'acorn_projects', 'add_member_of_goal', {
      goal_member: {
        goal_address: addr.Ok.goal.address,
        agent_address: alice.info('projects').agentAddress,
        unix_timestamp: Date.now()
      }
    })
    await s.consistency()
    await alice.call('projects', 'acorn_projects', 'add_member_of_goal', {
      goal_member: {
        goal_address: addr2.Ok.goal.address,
        agent_address: alice.info('projects').agentAddress,
        unix_timestamp: Date.now()
      }
    })
    await s.consistency()
    await alice.call('projects', 'acorn_projects', 'update_goal', {
      goal: {
        content: 'sample content2',
        user_hash: alice.info('projects').agentAddress,
        timestamp_created: Date.now(),
        hierarchy: 'Root',
        status: 'Uncertain',
        description: '33',
        time_frame: {
          from_date: Date.now(),
          to_date: Date.parse('Aug 9, 2020')
        }
      },
      address: addr.Ok.goal.address
    })
    await s.consistency()
    const history1 = await alice.call(
      'projects',
      'acorn_projects',
      'history_of_goal',
      {
        address: addr.Ok.goal.address
      }
    )
    await alice.call('projects', 'acorn_projects', 'archive_goal', {
      address: addr.Ok.goal.address
    })
    await s.consistency()

    t.equal(history1.Ok.entries.length, 2)
  }
)

orchestrator.registerScenario('create goal test', async (s, t) => {
  // the 'true' is for 'start', which means boot the Conductors
  const { alice } = await s.players({ alice: projectsConfig }, true)
  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const addr = await alice.call('projects', 'acorn_projects', 'create_goal', {
    goal: {
      content: 'sample content',
      user_hash: alice.info('projects').agentAddress,
      timestamp_created: Date.now(),
      hierarchy: 'Branch',
      status: 'Uncertain',
      description: ''
    },
    maybe_goal_edge_input: null
  })

  // Wait for all network activity to
  await s.consistency()

  const result = await alice.call(
    'projects',
    'acorn_projects',
    'fetch_goals',
    {}
  )
  // check for equality of the actual and expected results
  t.deepEqual(addr.Ok.goal, result.Ok[0])
})

orchestrator.registerScenario(
  'two agent test create, update and archive goals ',
  async (s, t) => {
    // the 'true' is for 'start', which means boot the Conductors
    const { alice, bob, alex } = await s.players(
      { alice: projectsConfig, bob: projectsConfig, alex: projectsConfig },
      true
    )
    const time2 = Date.now()
    // Make a call to a Zome function
    // indicating the function, and passing it an input
    const goal = await alice.call('projects', 'acorn_projects', 'create_goal', {
      goal: {
        content: 'sample content',
        user_hash: alice.info('projects').agentAddress,
        timestamp_created: time2,
        hierarchy: 'Branch',
        status: 'Uncertain',
        description: ''
      },
      maybe_goal_edge_input: null
    })

    const goal2 = await bob.call('projects', 'acorn_projects', 'create_goal', {
      goal: {
        content: 'sample content',
        user_hash: bob.info('projects').agentAddress,
        timestamp_created: Date.now(),
        hierarchy: 'Branch',
        status: 'Uncertain',
        description: '',
        time_frame: {
          from_date: Date.now(),
          to_date: Date.parse('Aug 9, 2020')
        }
      },
      maybe_goal_edge_input: {
        parent_address: goal.Ok.goal.address,
        randomizer: Date.now()
      }
    })

    // Wait for all network activity to
    await s.consistency()
    const time = Date.now()
    const result_bob = await bob.call(
      'projects',
      'acorn_projects',
      'update_goal',
      {
        goal: {
          content: 'sample content2',
          user_hash: alice.info('projects').agentAddress,
          timestamp_created: time,
          hierarchy: 'Root',
          status: 'Uncertain',
          description: '33',
          time_frame: null
        },
        address: goal.Ok.goal.address
      }
    )
    // check for equality of the actual and expected results
    await s.consistency()
    const result_alex = await alex.call(
      'projects',
      'acorn_projects',
      'add_member_of_goal',
      {
        goal_member: {
          goal_address: goal.Ok.goal.address,
          agent_address: alice.info('projects').agentAddress,
          unix_timestamp: Date.now()
        }
      }
    )
    const result_alex4 = await alex.call(
      'projects',
      'acorn_projects',
      'add_vote_of_goal',
      {
        goal_vote: {
          goal_address: goal.Ok.goal.address,
          urgency: 0.5,
          importance: 0.5,
          impact: 0.5,
          effort: 0.5,
          agent_address: alice.info('projects').agentAddress,
          unix_timestamp: Date.now()
        }
      }
    )
    await s.consistency()

    const result_alex2 = await alex.call(
      'projects',
      'acorn_projects',
      'fetch_goal_members',
      {}
    )
    const result_alex5 = await alex.call(
      'projects',
      'acorn_projects',
      'fetch_goal_votes',
      {}
    )
    const result_alice = await alice.call(
      'projects',
      'acorn_projects',
      'fetch_goals',
      {}
    )
    t.isNotEqual(goal2.Ok.maybe_edge, null)
    t.equal(result_alice.Ok.length, 2)
    t.deepEqual(result_bob.Ok.entry, {
      content: 'sample content2',
      user_hash: alice.info('projects').agentAddress,
      user_edit_hash: bob.info('projects').agentAddress,
      timestamp_created: time2,
      timestamp_updated: time,
      hierarchy: 'Root',
      status: 'Uncertain',
      tags: null,
      description: '33',
      time_frame: null
    })
    t.deepEqual(result_alex.Ok.entry, result_alex2.Ok[0].entry)
    t.deepEqual(result_alex4.Ok.entry, result_alex5.Ok[0].entry)
    const result_alex7 = await alex.call(
      'projects',
      'acorn_projects',
      'update_goal_vote',
      {
        goal_vote: {
          goal_address: goal.Ok.goal.address,
          urgency: 0,
          importance: 0,
          impact: 0,
          effort: 0,
          agent_address: alice.info('projects').agentAddress,
          unix_timestamp: Date.now()
        },
        address: result_alex4.Ok.address
      }
    )
    await s.consistency()
    const result_alex8 = await alex.call(
      'projects',
      'acorn_projects',
      'fetch_goal_votes',
      {}
    )

    t.deepEqual(result_alex7.Ok.entry, result_alex8.Ok[0].entry)
    const result_alice2 = await alice.call(
      'projects',
      'acorn_projects',
      'archive_goal',
      { address: goal.Ok.goal.address }
    )
    await s.consistency()
    const result_alice3 = await alice.call(
      'projects',
      'acorn_projects',
      'fetch_edges',
      {}
    )
    const result_bob2 = await bob.call(
      'projects',
      'acorn_projects',
      'fetch_goals',
      {}
    )

    const result_alex3 = await alex.call(
      'projects',
      'acorn_projects',
      'fetch_goal_members',
      {}
    )
    const result_alex6 = await alex.call(
      'projects',
      'acorn_projects',
      'fetch_goal_votes',
      {}
    )

    t.equal(result_alice2.Ok.address, goal.Ok.goal.address)
    t.equal(result_alice3.Ok.length, 0)
    t.equal(result_alex3.Ok.length, 0)
    t.equal(result_alex6.Ok.length, 0)

    t.equal(result_bob2.Ok.length, 1)
  }
)

orchestrator.registerScenario(
  'test create, fetch, update, then re-fetch goals',
  async (s, t) => {
    // the 'true' is for 'start', which means boot the Conductors
    const { alice } = await s.players({ alice: projectsConfig }, true)
    const create_goal = await alice.call(
      'projects',
      'acorn_projects',
      'create_goal',
      {
        goal: {
          content: 'sample content',
          user_hash: alice.info('projects').agentAddress,
          timestamp_created: Date.now(),
          hierarchy: 'Branch',
          status: 'Uncertain',
          description: ''
        },
        maybe_goal_edge_input: null
      }
    )
    await s.consistency()
    const first_fetch_goals_result = await alice.call(
      'projects',
      'acorn_projects',
      'fetch_goals',
      {}
    )
    await s.consistency()
    const time = Date.now()
    const update_goal = await alice.call(
      'projects',
      'acorn_projects',
      'update_goal',
      {
        goal: {
          content: 'sample content2',
          user_hash: alice.info('projects').agentAddress,
          timestamp_created: time,
          hierarchy: 'Root',
          status: 'Uncertain',
          description: '33'
        },
        address: create_goal.Ok.goal.address
      }
    )
    await s.consistency()
    const second_fetch_goals_result = await alice.call(
      'projects',
      'acorn_projects',
      'fetch_goals',
      {}
    )
    t.equal(
      first_fetch_goals_result.Ok[0].address,
      second_fetch_goals_result.Ok[0].address
    )
  }
)

orchestrator.registerScenario('alex and alice are commenting', async (s, t) => {
  // the 'true' is for 'start', which means boot the Conductors
  const { alice, alex } = await s.players(
    { alice: projectsConfig, alex: projectsConfig },
    true
  )
  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const goal = await alice.call('projects', 'acorn_projects', 'create_goal', {
    goal: {
      content: 'sample content',
      user_hash: alice.info('projects').agentAddress,
      timestamp_created: Date.now(),
      hierarchy: 'Branch',
      status: 'Uncertain',
      description: ''
    },
    maybe_goal_edge_input: null
  })
  const comment1 = await alice.call(
    'projects',
    'acorn_projects',
    'add_comment_of_goal',
    {
      goal_comment: {
        goal_address: goal.Ok.goal.address,
        content: 'hola mundo',
        agent_address: alice.info('projects').agentAddress,
        unix_timestamp: Date.now()
      }
    }
  )
  const comment2 = await alex.call(
    'projects',
    'acorn_projects',
    'add_comment_of_goal',
    {
      goal_comment: {
        goal_address: goal.Ok.goal.address,
        content: 'this is a test',
        agent_address: alex.info('projects').agentAddress,
        unix_timestamp: Date.now()
      }
    }
  )
  await s.consistency()
  const update = await alice.call(
    'projects',
    'acorn_projects',
    'update_goal_comment',
    {
      goal_comment: {
        goal_address: goal.Ok.goal.address,
        content: 'hello world',
        agent_address: alice.info('projects').agentAddress,
        unix_timestamp: Date.now()
      },
      address: comment1.Ok.address
    }
  )
  await s.consistency()
  await alex.call('projects', 'acorn_projects', 'archive_comment_of_goal', {
    address: comment2.Ok.address
  })
  await s.consistency()
  // Wait for all network activity to
  const fetch = await alice.call(
    'projects',
    'acorn_projects',
    'fetch_goal_comments',
    {}
  )
  t.equal(fetch.Ok.length, 1)
  t.deepEqual(fetch.Ok[0].entry, update.Ok.entry)
  await alice.call('projects', 'acorn_projects', 'archive_goal', {
    address: goal.Ok.goal.address
  })
  await s.consistency()
  const fetch2 = await alice.call(
    'projects',
    'acorn_projects',
    'fetch_goal_comments',
    {}
  )
  t.equal(fetch2.Ok.length, 0)
})

orchestrator.run()
