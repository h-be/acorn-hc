/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import {
  HOVER_GOAL,
  UNHOVER_GOAL,
} from './actions'

const defaultState = {
  hoveredGoal: null
}

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case HOVER_GOAL:
      return {
        ...state,
        hoveredGoal: payload
      }
    case UNHOVER_GOAL:
      return {
        ...state,
        hoveredGoal: null
      }
    default:
      return state
  }
}