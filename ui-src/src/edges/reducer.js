/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/
import _ from 'lodash'

import { createEdge, fetchEdges, archiveEdge } from './actions'
import { createGoal, archiveGoal } from '../goals/actions'

const defaultState = {}

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case createEdge.success().type:
      return {
        ...state,
        [payload.address]: {
          ...payload.entry,
          address: payload.address
        }
      }
    case fetchEdges.success().type:
        // payload is [ { entry: { key: val }, address: 'QmAsdFg' }, ... ]
        const mapped = payload.map(r => {
          return {
            ...r.entry,
            address: r.address
          }
        })
        // mapped is [ { key: val, address: 'QmAsdFg' }, ...]
        const newVals = _.keyBy(mapped, 'address')
        // combines pre-existing values of the object with new values from
        // Holochain fetch
        return {
          ...state,
          ...newVals
        }
    case archiveEdge.success().type:
        return  _.pickBy(state, (value, key) => key !== payload )
    case archiveGoal.success().type:
        // filter out the Edges whose addresses are listed as having been
        // archived on account of having archived one of the Goals it links
        return  _.pickBy(state, (value, key) => payload.archived_edges.indexOf(key) === -1)
    case createGoal.success().type:
        if (payload.maybe_edge) {
          return {
            ...state,
            [payload.maybe_edge.address]: {
              ...payload.maybe_edge.entry,
              address: payload.maybe_edge.address
            }
          }
        }
    default:
      return state
  }
}
