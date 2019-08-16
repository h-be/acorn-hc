/*
  There should be an actions.js file in every
  feature folder, and it should start with a list
  of constants defining all the types of actions
  that can be taken within that feature.
*/

/* constants */
const SELECT_GOAL = 'SELECT_GOAL'
const UNSELECT_GOAL = 'UNSELECT_GOAL'
const UNSELECT_ALL = 'UNSELECT_ALL'

/* action creator functions */

function selectGoal(address) {
  return {
    type: SELECT_GOAL,
    payload: address
  }
}

function unselectGoal(address) {
  return {
    type: UNSELECT_GOAL,
    payload: address
  }
}

function unselectAll() {
  return {
    type: UNSELECT_ALL
  }
}

export {
  SELECT_GOAL,
  UNSELECT_ALL,
  UNSELECT_GOAL,
  selectGoal,
  unselectAll,
  unselectGoal
}
