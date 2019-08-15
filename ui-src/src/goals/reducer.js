/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import { createGoal } from './actions'

const defaultState = [
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

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case createGoal.success().type:
      return state.concat([payload])
    default:
      return state
  }
}
