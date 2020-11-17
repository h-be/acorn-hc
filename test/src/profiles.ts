import { Config } from '@holochain/tryorama'
import { ScenarioApi } from '@holochain/tryorama/lib/api'
import * as _ from 'lodash'
import path from 'path'
import { delay } from './timer'

const config = Config.gen()
const profilesDnaPath = path.join(__dirname, '../../dnas/profiles/profiles.dna.gz')

module.exports = (orchestrator) => {
  orchestrator.registerScenario('profiles test', async (s: ScenarioApi, t) => {
    const [conductor1] = await s.players([config])
    const [[profileHapp]] = await conductor1.installAgentsHapps([[[profilesDnaPath]]])
    const [profilesCell] = profileHapp.cells
    // fetch_agent_address
    const agent_address = await profilesCell.call(
      'acorn_profiles',
      'fetch_agent_address',
    )

    const profile = {
      first_name: 'c',
      last_name: 't',
      handle: 'ct',
      status: 'Online',
      avatar_url: 'test',
      address: agent_address,
    }
    const create_whoami = await profilesCell.call(
      'acorn_profiles',
      'create_whoami',
      profile
    )

    const fetchAgentsResult = await profilesCell.call(
      'acorn_profiles',
      'fetch_agents',
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
    const update_whoami = await profilesCell.call(
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
    const whoami2 = await profilesCell.call(
      'acorn_profiles',
      'whoami',
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
    await profilesCell.call(
      'acorn_profiles',
      'update_whoami',
      {
        entry: profile3,
        address: create_whoami.address,
      }
    )
    await delay(2000)
    const whoami3 = await profilesCell.call(
      'acorn_profiles',
      'whoami',
    )
    t.deepEqual(whoami3.entry, profile3)
  })
}
