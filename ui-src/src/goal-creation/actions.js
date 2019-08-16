/*
  There should be an actions.js file in every
  feature folder, and it should start with a list
  of constants defining all the types of actions
  that can be taken within that feature.
*/

/* constants */
const OPEN_GOAL_CREATOR = 'OPEN_GOAL_CREATOR'
const CLOSE_GOAL_CREATOR = 'CLOSE_GOAL_CREATOR'
const SET_G_KEYDOWN = 'SET_G_KEYDOWN'
const UNSET_G_KEYDOWN = 'UNSET_G_KEYDOWN'
const UPDATE_CONTENT = 'UPDATE_CONTENT'

/* action creator functions */

function openGoalCreator(x, y) {
   return {
    type: OPEN_GOAL_CREATOR,
    payload: {
      x,
      y
    }
  }
}

function closeGoalCreator() {
   return {
    type: CLOSE_GOAL_CREATOR
  }
}

function setGKeyDown() {
   return {
    type: SET_G_KEYDOWN
  }
}

function unsetGKeyDown() {
   return {
    type: UNSET_G_KEYDOWN
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
  SET_G_KEYDOWN,
  UNSET_G_KEYDOWN,
  UPDATE_CONTENT,
  openGoalCreator,
  closeGoalCreator,
  setGKeyDown,
  unsetGKeyDown,
  updateContent
}
