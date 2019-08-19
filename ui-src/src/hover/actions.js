/*
  There should be an actions.js file in every
  feature folder, and it should start with a list
  of constants defining all the types of actions
  that can be taken within that feature.
*/

/* constants */
const HOVER_GOAL = 'HOVER_GOAL'
const UNHOVER_GOAL = 'UNHOVER_GOAL'

/* action creator functions */

function hoverGoal(address) {
  return {
    type: HOVER_GOAL,
    payload: address
  }
}

function unhoverGoal() {
  return {
    type: UNHOVER_GOAL
  }
}

export {
  HOVER_GOAL,
  UNHOVER_GOAL,
  hoverGoal,
  unhoverGoal
}
