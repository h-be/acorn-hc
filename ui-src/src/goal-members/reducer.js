/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/
import _ from 'lodash'

import { addMemberOfGoal, fetchGoalMembers, archiveMemberOfGoal } from './actions'

const defaultState = {}

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case addMemberOfGoal.success().type:
      return {
        ...state,
        [payload.address]: {
          ...payload.entry,
          address: payload.address
        }
      }
    case fetchGoalMembers.success().type:
        // payload is [ { entry: { key: val }, address: 'QmAsdFg' }, ... ]
        const mapped = payload.map(r => {
          return {
            ...r.entry,
            address: r.address
          }
        })
        // mapped is [ { key: val, address: 'QmAsdFg' }, ...]
        const newVals = _.keyBy(mapped, 'address')
        // combines pre-existing values of the object with new values from
        // Holochain fetch
        return {
          ...state,
          ...newVals
        }
    case archiveMemberOfGoal.success().type:
        return  _.pickBy(state, (value, key) => key !== payload )
    default:
      return state
  }
}
