/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import { CREATE_GOAL } from './actions'

const defaultState = []

export default function createGoal(state = defaultState, action) {
  switch (action.type) {
    case CREATE_GOAL:
      return state.concat([action.title])
    default:
      return state
  }
}
