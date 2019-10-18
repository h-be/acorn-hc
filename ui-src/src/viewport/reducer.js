/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/
import {
  coordsPageToCanvas, coordsCanvasToPage
} from '../drawing/coordinateSystems'
import {
  CHANGE_TRANSLATE,
  CHANGE_SCALE
} from './actions'

const defaultState = {
  translate: {
    x: 0,
    y: 0
  },
  scale: 1
}

export default function (state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case CHANGE_TRANSLATE:
      return {
        ...state,
        translate: {
          x: state.translate.x + payload.x,
          y: state.translate.y + payload.y
        }
      }
    case CHANGE_SCALE:
      const { zoom, mouseX, mouseY } = payload
      if (state.scale * zoom < 0.3 || state.scale * zoom > 2.5) {
        return state
      }
      // https://stackoverflow.com/a/20821545/2132755 helped
      return {
        ...state,
        scale: state.scale * zoom,
        translate: {
          x: mouseX - (mouseX - state.translate.x) * zoom,
          y: mouseY - (mouseY - state.translate.y) * zoom
        }
      }
    default:
      return state
  }
}
