/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import { createGoal, fetchGoals } from './actions'

const defaultState = []

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case createGoal.success().type:
      console.log("Goals state reducer found a createGoal!")
      console.log(payload)
      return state.concat([payload])
    case fetchGoals.success().type:
      console.log("Goals state reducer found a fetchGoals!")
      console.log(payload)
      return state.concat(payload)
    default:
      return state
  }
}
