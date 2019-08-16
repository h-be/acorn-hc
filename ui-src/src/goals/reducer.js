/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/
import _ from 'lodash'

import { createGoal, fetchGoals } from './actions'

const defaultState = {}

/*
[
  {
    content: "Small incomplete",
    user_hash: "Boop",
    unix_timestamp: 412,
    complete: false,
    certain: true,
    small: true
  },
  {
    content: "Other thing",
    user_hash: "Boop",
    unix_timestamp: 412,
    complete: false,
    certain: true,
    small: true
  },
  {
    content: "Small complete",
    user_hash: "Boop",
    unix_timestamp: 413,
    complete: true,
    certain: true,
    small: true
  }
]
*/

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case createGoal.success().type:
      return {
        ...state,
        [payload.address]: payload.goal
      }
    case fetchGoals.success().type:
      // payload is [ { goal: { key: val }, address: 'asdfy' }, ... ]
      const mapped = payload.map(r => {
        return {
          ...r.goal,
          address: r.address
        }
      })
      // mapped is [ { key: val, address: 'asdfy' }, ...]
      const newVals = _.keyBy(mapped, 'address')
      // combines pre-existing values of the object with new values from
      // Holochain fetch
      return {
        ...state,
        ...newVals
      }
    default:
      return state
  }
}
