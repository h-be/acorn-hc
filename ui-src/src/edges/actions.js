/*
  There should be an actions.js file in every
  feature folder, and it should start with a list
  of constants defining all the types of actions
  that can be taken within that feature.
*/

import { createHolochainZomeCallAsyncAction } from '@holochain/hc-redux-middleware'

/* action creator functions */

const DEVELOPMENT_INSTANCE_NAME = 'test-instance'
const ZOME_NAME = 'my_zome' // TODO: change this

const createGoal = createHolochainZomeCallAsyncAction(DEVELOPMENT_INSTANCE_NAME, ZOME_NAME, 'create_goal')

export {
  createGoal
}
