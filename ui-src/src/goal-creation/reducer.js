/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import {
  OPEN_GOAL_CREATOR,
  CLOSE_GOAL_CREATOR
} from './actions'

const defaultState = {
  isOpen: true
}

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case OPEN_GOAL_CREATOR:
      return {
        ...state,
        isOpen: true
      }
    case CLOSE_GOAL_CREATOR:
      return {
        ...state,
        isOpen: false
      }
    default:
      return state
  }
}
