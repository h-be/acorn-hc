/*
  There should be an actions.js file in every
  feature folder, and it should start with a list
  of constants defining all the types of actions
  that can be taken within that feature.
*/

import { createHolochainZomeCallAsyncAction } from '@holochain/hc-redux-middleware'

import {
  DEVELOPMENT_INSTANCE_NAME,
  ZOME_NAME
} from '../holochainConfig'

/* action creator functions */

const createEdge = createHolochainZomeCallAsyncAction(DEVELOPMENT_INSTANCE_NAME, ZOME_NAME, 'create_edge')
const fetchEdges = createHolochainZomeCallAsyncAction(DEVELOPMENT_INSTANCE_NAME, ZOME_NAME, 'fetch_edges')
const archiveEdge = createHolochainZomeCallAsyncAction(DEVELOPMENT_INSTANCE_NAME, ZOME_NAME, 'archive_edge')

export {
  createEdge,
  fetchEdges,
  archiveEdge
}
