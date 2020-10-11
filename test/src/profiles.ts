import { Config } from '@holochain/tryorama'
import * as _ from 'lodash'
import { delay } from './timer'

// Configure a conductor with two identical DNAs,
// differentiated by UUID, nicknamed "alice" and "bobbo"
const config = Config.gen(
  {
    alice: Config.dna('../dnas/profiles/profiles.dna.gz', null),
    bobbo: Config.dna('../dnas/profiles/profiles.dna.gz', null),
  }
  // { logger: Config.logger(true) }
)

module.exports = (orchestrator) => {
  orchestrator.registerScenario('profiles test', async (s, t) => {
    // spawn the conductor process
    const { conductor } = await s.players({ conductor: config })
    await conductor.spawn()

    // fetch_agent_address
    const agent_address = await conductor.call(
      'bobbo',
      'acorn_profiles',
      'fetch_agent_address',
      null
    )

    const profile = {
      first_name: 'c',
      last_name: 't',
      handle: 'ct',
      status: 'Online',
      avatar_url: 'test',
      address: agent_address,
    }
    const create_whoami = await conductor.call(
      'bobbo',
      'acorn_profiles',
      'create_whoami',
      profile
    )

    const fetchAgentsResult = await conductor.call(
      'bobbo',
      'acorn_profiles',
      'fetch_agents',
      null
    )

    t.deepEqual(fetchAgentsResult, [profile])

    // UPDATE WHOAMI
    const profile2 = {
      first_name: 'c',
      last_name: 't',
      handle: 'ct',
      status: 'Offline',
      avatar_url: 'test',
      address: agent_address,
    }
    const update_whoami = await conductor.call(
      'bobbo',
      'acorn_profiles',
      'update_whoami',
      {
        entry: profile2,
        address: create_whoami.address,
      }
    )
    t.deepEqual(update_whoami.entry, profile2)

    await delay(2000)

    // WHOAMI
    const whoami2 = await conductor.call(
      'bobbo',
      'acorn_profiles',
      'whoami',
      null
    )
    t.deepEqual(whoami2.entry, {
      ...profile2,
      status: 'Offline',
    })

    // UPDATE WHOAMI Again
    const profile3 = {
      first_name: 'c',
      last_name: 't',
      handle: 'ct',
      status: 'Away',
      avatar_url: 'test',
      address: agent_address,
    }
    await conductor.call('bobbo', 'acorn_profiles', 'update_whoami', {
      entry: profile3,
      address: create_whoami.address,
    })
    await delay(2000)
    const whoami3 = await conductor.call(
      'bobbo',
      'acorn_profiles',
      'whoami',
      null
    )
    t.deepEqual(whoami3.entry, profile3)
  })
}
