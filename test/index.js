/// NB: The try-o-rama config patterns are still not quite stabilized.
/// See the try-o-rama README [https://github.com/holochain/try-o-rama]
/// for a potentially more accurate example

const path = require('path')
const tape = require('tape')

const { Orchestrator, Config, tapeExecutor, singleConductor, combine  } = require('@holochain/try-o-rama')

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
    singleConductor,

    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require('tape'))
  ),

  globalConfig: {
    logger: true,
    network: 'memory',  // must use singleConductor middleware if using in-memory network
  },

  // the following are optional:

  waiter: {
    softTimeout: 5000,
    hardTimeout: 10000,
  },
})

const conductorConfig = {
  instances: {
    myInstanceName: Config.dna(dnaPath, 'scaffold-test')
  }
}

orchestrator.registerScenario("description of example test", async (s, t) => {

  const {alice, bob} = await s.players({alice: conductorConfig, bob: conductorConfig})
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

  const result = await alice.call("myInstanceName", "holo_acorn", "fetch_goal", {"address": addr.Ok.goal.address})
  console.log(addr)
  // check for equality of the actual and expected results
  t.deepEqual(result, { Ok: {"content":"sample content","user_hash": "HcSciJuZ4M4e4Iew7zsJCU8jdTxypahgfE5BJGzQANqhhmnjRa9vTu4snh44kuz","unix_timestamp":f,"hierarchy": "Branch","status": "Uncertain"}})
})

orchestrator.run()
