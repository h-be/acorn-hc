/*
  There should be an actions.js file in every
  feature folder, and it should start with a list
  of constants defining all the types of actions
  that can be taken within that feature.
*/

/* constants */
const CHANGE_TRANSLATE = 'CHANGE_TRANSLATE'

/* action creator functions */

function changeTranslate(x, y) {
  return {
    type: CHANGE_TRANSLATE,
    payload: {
      x,
      y
    }
  }
}

export {
  CHANGE_TRANSLATE,
  changeTranslate
}
