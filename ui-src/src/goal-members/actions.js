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

const addMemberOfGoal = createHolochainZomeCallAsyncAction(DEVELOPMENT_INSTANCE_NAME, ZOME_NAME, 'add_member_of_goal')
const archiveMemberOfGoal = createHolochainZomeCallAsyncAction(DEVELOPMENT_INSTANCE_NAME, ZOME_NAME, 'archive_member_of_goal')
const fetchGoalMembers = createHolochainZomeCallAsyncAction(DEVELOPMENT_INSTANCE_NAME, ZOME_NAME, 'fetch_goal_members')

export {
  addMemberOfGoal,
  fetchGoalMembers,
  archiveMemberOfGoal
}
