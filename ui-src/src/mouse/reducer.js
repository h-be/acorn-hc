/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import {
  SET_MOUSEDOWN,
  UNSET_MOUSEDOWN
} from './actions'

const defaultState = {
  mousedown: false
}

export default function(state = defaultState, action) {
  const { type } = action
  switch (type) {
    case SET_MOUSEDOWN:
      return {
        ...state,
        mousedown: true
      }
    case UNSET_MOUSEDOWN:
      return {
        ...state,
        mousedown: false
      }
    default:
      return state
  }
}
