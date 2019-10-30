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
    logger: {
      type: 'info'
    },
    // network: 'memory'
    network: {
      type: 'sim2h',
      sim2h_url: 'wss://localhost:9000'
    }  // must use singleConductor middleware if using in-memory network
  },

  // the following are optional:

})

const conductorConfig = {
  instances: {
    myInstanceName: Config.dna(dnaPath, 'scaffold-test')
  }
}

orchestrator.registerScenario("description of example test", async (s, t) => {

   // the 'true' is for 'start', which means boot the Conductors
  const {alice, bob} = await s.players({alice: conductorConfig, bob: conductorConfig}, true)
  const f = Date.now()
  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const addr = await alice.call("myInstanceName", "holo_acorn", "create_goal", {"goal" : {"content":"sample content",
  'user_hash': "HcSciJuZ4M4e4Iew7zsJCU8jdTxypahgfE5BJGzQANqhhmnjRa9vTu4snh44kuz",
  "unix_timestamp":f,
  "hierarchy": "Branch",
  "status": "Uncertain"},"maybe_parent_address":null})

  // Wait for all network activity to
  await s.consistency()

  const result = await alice.call("myInstanceName", "holo_acorn", "fetch_goals", {})
  // check for equality of the actual and expected results
  t.deepEqual(addr.Ok.goal, result.Ok[0])
})

orchestrator.run()
