/*
  There should be an actions.js file in every
  feature folder, and it should start with a list
  of constants defining all the types of actions
  that can be taken within that feature.
*/

/* constants */
const CREATE_GOAL = "CREATE_GOAL"

/* action creator functions */
function createGoal(title) {
  return {
    type: CREATE_GOAL,
    title // title: title
  }
}

export {
  CREATE_GOAL,
  createGoal
}
