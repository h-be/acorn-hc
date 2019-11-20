/// NB: The try-o-rama config patterns are still not quite stabilized.
/// See the try-o-rama README [https://github.com/holochain/try-o-rama]
/// for a potentially more accurate example

const path = require('path')

const { Orchestrator, Config, tapeExecutor, singleConductor, combine, callSync  } = require('@holochain/try-o-rama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/acorn-hc.dna.json")

const orchestrator = new Orchestrator({
  middleware: combine(
    // squash all instances from all conductors down into a single conductor,
    // for in-memory testing purposes.
    // Remove this middleware for other "real" network types which can actually
    // send messages across conductors
    // singleConductor,
    // callSync,
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require('tape'))
  ),

  globalConfig: {
    logger: false,
    network: {
      type: 'sim2h',
      sim2h_url: 'wss://sim2h.holochain.org:9000',
    },  // must use singleConductor middleware if using in-memory network
  },

  // the following are optional:

})

const conductorConfig = {
  instances: {
    acorn_hc: Config.dna(dnaPath, 'acorn_hc')
  }
}

orchestrator.registerScenario("create profile test", async (s, t) => {

  // the 'true' is for 'start', which means boot the Conductors
  const {alice} = await s.players({alice: conductorConfig}, true)
  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const getProfile = await alice.call("acorn_hc", "holo_acorn", "create_whoami", {"profile" : {"first_name":"alice",
  'last_name': "velandia",
  "handle": "Branch",
  "avatar_url":"",
  "address": alice._instances.acorn_hc.agentAddress}})
 
  // Wait for all network activity to
  await s.consistency()
 
  const result = await alice.call("acorn_hc", "holo_acorn", "whoami", {})
  // check for equality of the actual and expected results
  t.deepEqual(getProfile, result)
})

orchestrator.registerScenario("create goal test", async (s, t) => {

  // the 'true' is for 'start', which means boot the Conductors
 const {alice} = await s.players({alice: conductorConfig}, true)
 // Make a call to a Zome function
 // indicating the function, and passing it an input
 const addr = await alice.call("acorn_hc", "holo_acorn", "create_goal", {"goal" : {"content":"sample content",
 'user_hash': alice._instances.acorn_hc.agentAddress,
 "unix_timestamp":Date.now(),
 "hierarchy": "Branch",
 "status": "Uncertain"},"maybe_parent_address":null})

 // Wait for all network activity to
 await s.consistency()

 const result = await alice.call("acorn_hc", "holo_acorn", "fetch_goals", {})
 // check for equality of the actual and expected results
 t.deepEqual(addr.Ok.goal, result.Ok[0])
})

orchestrator.registerScenario("two agent test", async (s, t) => {
  // the 'true' is for 'start', which means boot the Conductors
  const {alice,bob} = await s.players({alice: conductorConfig,bob: conductorConfig}, true)
  // Make a call to a Zome function
  // indicating the function, and passing it an input
   await alice.call("acorn_hc", "holo_acorn", "create_whoami", {"profile" : {"first_name":"alice",
  'last_name': "velandia",
  "handle": "Branch",
  "avatar_url":"",
  "address": alice._instances.acorn_hc.agentAddress}})
   await bob.call("acorn_hc", "holo_acorn", "create_whoami", {"profile" : {"first_name":"bob",
  'last_name': "romero",
  "handle": "Branch",
  "avatar_url":"",
  "address": bob._instances.acorn_hc.agentAddress}})
 
  // Wait for all network activity to
  await s.consistency()
  const result_alice = await alice.call("acorn_hc", "holo_acorn", "fetch_agent_address", {})
  const result_bob = await bob.call("acorn_hc", "holo_acorn", "fetch_agent_address", {})
  // check for equality of the actual and expected results
  const result = await alice.call("acorn_hc", "holo_acorn", "fetch_agents", {})
  
  t.equal(result.Ok.length,2)
  t.isNotDeepEqual(result_alice.Ok, result_bob.Ok)
})

orchestrator.registerScenario("two agent test create, update and archive goals ", async (s, t) => {
  // the 'true' is for 'start', which means boot the Conductors
  const {alice,bob,alex} = await s.players({alice: conductorConfig,bob: conductorConfig,alex:conductorConfig}, true)
  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const goal = await alice.call("acorn_hc", "holo_acorn", "create_goal", {"goal" : {"content":"sample content",
 'user_hash': alice._instances.acorn_hc.agentAddress,
 "unix_timestamp":Date.now(),
 "hierarchy": "Branch",
 "status": "Uncertain"},"maybe_parent_address":null})
 const goal2 = await bob.call("acorn_hc", "holo_acorn", "create_goal", {"goal" : {"content":"sample content",
 'user_hash': bob._instances.acorn_hc.agentAddress,
 "unix_timestamp":Date.now(),
 "hierarchy": "Branch",
 "status": "Uncertain"},"maybe_parent_address":goal.Ok.goal.address})
 
  // Wait for all network activity to
  await s.consistency()
  const time=Date.now()
  const result_bob = await bob.call("acorn_hc", "holo_acorn", "update_goal", {"goal" : {"content":"sample content2",
  'user_hash': alice._instances.acorn_hc.agentAddress,
  "unix_timestamp":time,
  "hierarchy": "Root",
  "status": "Uncertain"},"address":goal.Ok.goal.address})
  // check for equality of the actual and expected results
  await s.consistency()
  const result_alex=await alex.call("acorn_hc","holo_acorn","add_member_of_goal",{"goal_member":{"goal_address":goal.Ok.goal.address,
    "agent_address":alice._instances.acorn_hc.agentAddress,"unix_timestamp":Date.now()}})
    await s.consistency()
  const result_alex2=await alex.call("acorn_hc","holo_acorn","fetch_goal_members",{})
  const result_alice= await alice.call("acorn_hc", "holo_acorn", "fetch_goals",{})
  t.isNotEqual(goal2.Ok.maybe_edge,null)
  t.equal(result_alice.Ok.length,2)
  t.deepEqual(result_bob.Ok.entry,{"content":"sample content2",
  'user_hash': alice._instances.acorn_hc.agentAddress,
  "unix_timestamp":time,
  "hierarchy": "Root",
  "status": "Uncertain"})
  t.deepEqual(result_alex.Ok.entry,result_alex2.Ok[0].entry)
  await s.consistency()
  const result_alice2= await alice.call("acorn_hc", "holo_acorn", "archive_goal",{"address":goal.Ok.goal.address})
  await s.consistency()
  const result_alice3= await alice.call("acorn_hc", "holo_acorn", "fetch_edges",{})
  const result_bob2= await bob.call("acorn_hc", "holo_acorn", "fetch_goals",{})
  const result_alex3=await alex.call("acorn_hc","holo_acorn","fetch_goal_members",{})

  t.equal(result_alice2.Ok.address,goal.Ok.goal.address)
  t.equal(result_alice3.Ok.length,0)
  t.equal(result_alex3.Ok.length,0)
  t.equal(result_bob2.Ok.length,1)



  
})

orchestrator.run()
