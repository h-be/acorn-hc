/*
  There should be an actions.js file in every
  feature folder, and it should start with a list
  of constants defining all the types of actions
  that can be taken within that feature.
*/

import { createHolochainZomeCallAsyncAction } from '@holochain/hc-redux-middleware'

import {
  DEVELOPMENT_INSTANCE_NAME,
  ZOME_NAME
} from '../holochainConfig'

/* action creator functions */

const createGoal = createHolochainZomeCallAsyncAction(DEVELOPMENT_INSTANCE_NAME, ZOME_NAME, 'create_goal')
const fetchGoals = createHolochainZomeCallAsyncAction(DEVELOPMENT_INSTANCE_NAME, ZOME_NAME, 'fetch_goals')
const archiveGoal = createHolochainZomeCallAsyncAction(DEVELOPMENT_INSTANCE_NAME, ZOME_NAME, 'archive_goal')
const updateGoal = createHolochainZomeCallAsyncAction(DEVELOPMENT_INSTANCE_NAME, ZOME_NAME, 'update_goal')


export {
  createGoal,
  fetchGoals,
  archiveGoal,
  updateGoal
}
