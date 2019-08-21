/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import {
  OPEN_GOAL_FORM,
  CLOSE_GOAL_FORM,
  UPDATE_CONTENT
} from './actions'

const defaultState = {
  editAddress: null,
  parentAddress: null,
  content: '',
  isOpen: false,
  xLoc: 0,
  yLoc: 0
}

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case UPDATE_CONTENT:
      return {
        ...state,
        content: payload
      }
    case OPEN_GOAL_FORM:
      return {
        ...state,
        isOpen: true,
        xLoc: payload.x,
        yLoc: payload.y,
        parentAddress: payload.parentAddress,
        editAddress: payload.editAddress
      }
    case CLOSE_GOAL_FORM:
      return {
        ...state,
        isOpen: false,
        content: '',
        parentAddress: null,
        editAddress: null
      }
    default:
      return state
  }
}
