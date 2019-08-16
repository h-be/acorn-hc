/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import { createEdge, fetchEdges } from './actions'

const defaultState = []

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case createEdge.success().type:
      return state.concat([payload])
    case fetchEdges.success().type:
        // TODO: this isn't perfect, could include duplicates
        return state.concat(payload)
    default:
      return state
  }
}
