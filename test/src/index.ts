import { Orchestrator } from '@holochain/tryorama'

const orchestrator = new Orchestrator<null>()

require('./profiles')(orchestrator)
require('./projects')(orchestrator)

orchestrator.run()

