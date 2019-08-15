/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

// import { createGoal } from './actions'

const defaultState = [
  {
    parent: 'Small incomplete',
    child: 'Small complete'
  }/* ,
  {
    parent: 'Small complete',
    child: 'Other thing'
  } */
]

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    // case createGoal.success().type:
    //   return state.concat([payload])
    default:
      return state
  }
}
