/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import {
  CHANGE_TRANSLATE
} from './actions'

const defaultState = {
  translate: {
    x: 0,
    y: 0
  }
}

export default function (state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case CHANGE_TRANSLATE:
      return {
        ...state,
        translate: {
          x: state.translate.x += payload.x,
          y: state.translate.y += payload.y
        }
      }
    default:
      return state
  }
}
