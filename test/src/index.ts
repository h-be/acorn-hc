import { Orchestrator } from '@holochain/tryorama'

const orchestrator = new Orchestrator()

require('./profiles')(orchestrator)
require('./projects')(orchestrator)

orchestrator.run()

