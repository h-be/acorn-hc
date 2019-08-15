/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import {
  OPEN_GOAL_CREATOR,
  CLOSE_GOAL_CREATOR,
  SET_G_KEYDOWN,
  UNSET_G_KEYDOWN
} from './actions'

const defaultState = {
  gKeyDown: false,
  isOpen: false,
  xLoc: 2,
  yLoc: 2
}

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case OPEN_GOAL_CREATOR:
      return {
        ...state,
        isOpen: true,
        xLoc: payload.x,
        yLoc: payload.y
      }
    case CLOSE_GOAL_CREATOR:
      return {
        ...state,
        isOpen: false
      }
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
    default:
      return state
  }
}
