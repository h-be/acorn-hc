/*
  There should be an actions.js file in every
  feature folder, and it should start with a list
  of constants defining all the types of actions
  that can be taken within that feature.
*/

/* constants */
const SET_MOUSEDOWN = 'SET_MOUSEDOWN'
const UNSET_MOUSEDOWN = 'UNSET_MOUSEDOWN'

/* action creator functions */

function setMousedown() {
  return {
   type: SET_MOUSEDOWN
 }
}

function unsetMousedown() {
  return {
   type: UNSET_MOUSEDOWN
 }
}

export {
  SET_MOUSEDOWN,
  UNSET_MOUSEDOWN,
  setMousedown,
  unsetMousedown
}
