/*
  There should be an actions.js file in every
  feature folder, and it should start with a list
  of constants defining all the types of actions
  that can be taken within that feature.
*/

/* constants */
const OPEN_GOAL_CREATOR = "OPEN_GOAL_CREATOR"
const CLOSE_GOAL_CREATOR = "CLOSE_GOAL_CREATOR"

/* action creator functions */

function openGoalCreator() {
   return {
    type: OPEN_GOAL_CREATOR
  }
}

function closeGoalCreator() {
   return {
    type: CLOSE_GOAL_CREATOR
  }
}

export {
  OPEN_GOAL_CREATOR,
  CLOSE_GOAL_CREATOR,
  openGoalCreator,
  closeGoalCreator
}
