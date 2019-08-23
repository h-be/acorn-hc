import {
  archiveGoal
} from './actions'
import {
  archiveEdge
} from '../edges/actions'

// TODO: maybe move this logic into Holochain?

export default function archiveHelper(store, toArchiveAddress) {
  let state = store.getState()

  // remove goal
  store.dispatch(archiveGoal.create({ address: toArchiveAddress  }))

  // remove all edges connecting to or from the removed goal
  const edgeAddressesArray = Object.keys(state.edges)
  const edgesAsArray = edgeAddressesArray.map(address => state.edges[address])
  edgesAsArray.forEach(({ parent_address, child_address, address }) => {
    if (toArchiveAddress === parent_address  || toArchiveAddress === child_address) {
      store.dispatch(archiveEdge.create({ address }))
    }
  })
}
