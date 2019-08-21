/*
  There should be an actions.js file in every
  feature folder, and it should start with a list
  of constants defining all the types of actions
  that can be taken within that feature.
*/

/* constants */
const OPEN_GOAL_FORM = 'OPEN_GOAL_FORM'
const CLOSE_GOAL_FORM = 'CLOSE_GOAL_FORM'
const UPDATE_CONTENT = 'UPDATE_CONTENT'

/* action creator functions */

// parentAddress is optional
function openGoalForm(x, y, editAddress, parentAddress) {
   return {
    type: OPEN_GOAL_FORM,
    payload: {
      editAddress,
      x,
      y,
      parentAddress
    }
  }
}

function closeGoalForm() {
   return {
    type: CLOSE_GOAL_FORM
  }
}

function updateContent(content) {
  return {
   type: UPDATE_CONTENT,
   payload: content
 }
}

export {
  OPEN_GOAL_FORM,
  CLOSE_GOAL_FORM,
  UPDATE_CONTENT,
  openGoalForm,
  closeGoalForm,
  updateContent
}
