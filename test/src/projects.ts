import { Config } from '@holochain/tryorama'
import * as _ from 'lodash'
import { delay } from './timer'

// Configure a conductor with two identical DNAs,
// differentiated by UUID, nicknamed "alice" and "bobbo"
const config = Config.gen(
  {
    alice: Config.dna('../dnas/projects/projects.dna.gz', null),
    bobbo: Config.dna('../dnas/projects/projects.dna.gz', null),
  }
  // { logger: Config.logger(true) }
)

module.exports = (orchestrator) => {
  orchestrator.registerScenario('projects test', async (s, t) => {
    // spawn the conductor process
    // const { conductor } = await s.players({ conductor: config })
    // await conductor.spawn()

  })
}
