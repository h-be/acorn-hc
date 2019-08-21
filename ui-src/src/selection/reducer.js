/*
  There should be a reducer.js file in every feature folder.
  It should define and export a function which takes a state
  and an action, applies that action to the state, and return
  a new state.
*/

import {
  SELECT_GOAL,
  UNSELECT_GOAL,
  UNSELECT_ALL
} from './actions'
import {
 archiveGoal
} from '../goals/actions'

const defaultState = {
  selectedGoals: []
}

// removes an item from an array without mutating original array
function arrayWithoutElement(array, elem) {
    var newArray = array.slice()
    var index = newArray.indexOf(elem)
    if (index > -1) {
        newArray.splice(index, 1)
    }
    return newArray
}

export default function(state = defaultState, action) {
  const { payload, type } = action
  switch (type) {
    case SELECT_GOAL:
      return {
        ...state,
        selectedGoals: state.selectedGoals.indexOf(payload) > -1
          ? state.selectedGoals.slice() // you should create a new copy of the array, regardless, because redux
          : state.selectedGoals.concat([payload]) // combine the existing list of selected with the new one to add
      }
    case UNSELECT_GOAL:
      return {
        ...state,
        selectedGoals: state.selectedGoals.filter(address => address !== payload)
      }
    case UNSELECT_ALL:
      return {
        ...state,
        selectedGoals: []
      }
    case archiveGoal.success().type:
      // unselect if the archived Goal was selected
      return state.selectedGoals.includes(payload) ? {
        ...state,
        selectedGoals: arrayWithoutElement(state.selectedGoals, payload)
      } : { ...state }
    default:
      return state
  }
}
