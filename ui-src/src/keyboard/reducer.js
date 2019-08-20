/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import {
  SET_G_KEYDOWN,
  UNSET_G_KEYDOWN,
  SET_SHIFT_KEYDOWN,
  UNSET_SHIFT_KEYDOWN
} from './actions'

const defaultState = {
  shiftKeyDown: false,
  gKeyDown: false
}

export default function(state = defaultState, action) {
  const { type } = action
  switch (type) {
    case SET_G_KEYDOWN:
      return {
        ...state,
        gKeyDown: true
      }
    case UNSET_G_KEYDOWN:
      return {
        ...state,
        gKeyDown: false
      }
    case SET_SHIFT_KEYDOWN:
      return {
        ...state,
        shiftKeyDown: true
      }
    case UNSET_SHIFT_KEYDOWN:
      return {
        ...state,
        shiftKeyDown: false
      }
    default:
      return state
  }
}
