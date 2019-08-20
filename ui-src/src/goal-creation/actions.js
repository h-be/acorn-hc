/*
  There should be an actions.js file in every
  feature folder, and it should start with a list
  of constants defining all the types of actions
  that can be taken within that feature.
*/

/* constants */
const OPEN_GOAL_CREATOR = 'OPEN_GOAL_CREATOR'
const CLOSE_GOAL_CREATOR = 'CLOSE_GOAL_CREATOR'
const UPDATE_CONTENT = 'UPDATE_CONTENT'

/* action creator functions */

// parentAddress is optional
function openGoalCreator(x, y, parentAddress) {
   return {
    type: OPEN_GOAL_CREATOR,
    payload: {
      x,
      y,
      parentAddress
    }
  }
}

function closeGoalCreator() {
   return {
    type: CLOSE_GOAL_CREATOR
  }
}

function updateContent(content) {
  return {
   type: UPDATE_CONTENT,
   payload: content
 }
}

export {
  OPEN_GOAL_CREATOR,
  CLOSE_GOAL_CREATOR,
  UPDATE_CONTENT,
  openGoalCreator,
  closeGoalCreator,
  updateContent
}
