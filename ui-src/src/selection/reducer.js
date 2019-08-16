/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import {
  SELECT_GOAL,
  UNSELECT_GOAL,
  UNSELECT_ALL
} from './actions'

const defaultState = {
  selectedGoals: []
}

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case SELECT_GOAL:
      return {
        ...state,
        selectedGoals: [payload]
      }
    case UNSELECT_GOAL:
      return {
        ...state,
        selectedGoals: state.selectedGoals.filter(address => address !== payload)
      }
    case UNSELECT_ALL:
      return {
        ...state,
        selectedGoals: []
      }
    default:
      return state
  }
}
