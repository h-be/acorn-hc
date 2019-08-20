/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import {
  OPEN_GOAL_CREATOR,
  CLOSE_GOAL_CREATOR,
  UPDATE_CONTENT
} from './actions'

const defaultState = {
  parentAddress: null,
  content: '',
  isOpen: false,
  xLoc: 2,
  yLoc: 2
}

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case UPDATE_CONTENT:
      return {
        ...state,
        content: payload
      }
    case OPEN_GOAL_CREATOR:
      return {
        ...state,
        isOpen: true,
        xLoc: payload.x,
        yLoc: payload.y,
        parentAddress: payload.parentAddress
      }
    case CLOSE_GOAL_CREATOR:
      return {
        ...state,
        isOpen: false,
        content: '',
        parentAddress: null
      }
    default:
      return state
  }
}
