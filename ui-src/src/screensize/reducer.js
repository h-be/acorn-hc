/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import {
  SET_SCREEN_DIMENSIONS
} from './actions'

const defaultState = {
  width: 0,
  height: 0
}

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case SET_SCREEN_DIMENSIONS:
      return {
        width: payload.width,
        height: payload.height
      }
    default:
      return state
  }
}
