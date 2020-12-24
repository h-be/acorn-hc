import { Config, Orchestrator } from '@holochain/tryorama'
import { ScenarioApi } from '@holochain/tryorama/lib/api'
import * as _ from 'lodash'
import path from 'path'
import { delay } from './timer'

const ZOME = 'acorn_projects'
const config = Config.gen()
const projectsDnaPath = path.join(
  __dirname,
  '../../dnas/projects/projects.dna.gz'
)
type Hash = Buffer

function newGoal(agentAddress: Hash, content: string) {
  return {
    content,
    description: 'Test Goal Description',
    user_hash: agentAddress,
    user_edit_hash: null,
    timestamp_created: Date.now(),
    timestamp_updated: null,
    hierarchy: 'Root',
    status: 'Uncertain',
    tags: null,
    time_frame: null,
  }
}

async function setup(scenario: ScenarioApi) {
  const [conductor] = await scenario.players([config])
  const [
    [
      {
        agent,
        cells: [projectsCell],
      },
    ],
  ] = await conductor.installAgentsHapps([[[projectsDnaPath]]])

  function callAlice(fn: string, payload?: any) {
    return projectsCell.call(ZOME, fn, payload)
  }

  // await delay(5000)

  return {
    callAlice,
    agentAddress: agent,
    conductor,
  }
}

module.exports = (orchestrator: Orchestrator<null>) => {
  orchestrator.registerScenario(
    'member api',
    async (scenario: ScenarioApi, tape) => {

      // conductor 1 starts up first
      const {
        callAlice,
        conductor: conductor1,
      } = await setup(scenario)

      const [{ address: agentAddress }] = await callAlice('fetch_members')
      const goal1 = newGoal(agentAddress, 'Test Goal 1')
      const createGoal1Result = await callAlice('create_goal', goal1)

      console.log('waiting 5 seconds')
      await delay(5000)
      console.log('done waiting')

      // after waiting, conductor 2 starts up
      const {
        callAlice: callBob,
        conductor: conductor2,
      } = await setup(scenario)

      const [{ address: agentAddress2 }] = await callBob('fetch_members')

      await scenario.shareAllNodes([conductor1, conductor2])

      await delay(5000)

      const goal2 = newGoal(agentAddress, 'Test Goal 2')
      const createGoal2Result = await callAlice('create_goal', goal2)

      const result = await callAlice('fetch_members')

      const sampleResult =
        'uhCAkmrkoAHPVf_eufG7eC5fm6QKrW5pPMoktvG5LOC0SnJ4vV1Uv'
      tape.equal(2, result.length)
      tape.ok(result[0].address)
      tape.equal(result[0].address.length, sampleResult.length)
    }
  )
}
