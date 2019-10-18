import React from 'react'
import PropTypes from 'prop-types'
import { connect } from 'react-redux'

import GoalForm from './GoalForm'
import Help from './Help'
import MultiEditBar from './MultiEditBar'
import HoverOverlay from './HoverOverlay'

function App(props) {
  const { hasSelection, hasHover, goalFormIsOpen, translate, scale } = props
  const transform = {
    transform: `matrix(${scale}, 0, 0, ${scale}, ${translate.x}, ${translate.y})`
  }
  return (
    <div>
      <Help />
      {hasSelection && <MultiEditBar />}
      <div style={transform}>
        {goalFormIsOpen && <GoalForm />}
        {hasHover && <HoverOverlay />}
      </div>
    </div>
  )
}

App.propTypes = {
  hasSelection: PropTypes.bool.isRequired, // whether or not there are Goals selected
  hasHover: PropTypes.bool.isRequired, // whether or not a Goal is hovered over
  goalFormIsOpen: PropTypes.bool.isRequired,
  translate: PropTypes.shape({
    x: PropTypes.number.isRequired,
    y: PropTypes.number.isRequired
  }),
  scale: PropTypes.number.isRequired
}

function mapStateToProps(state) {
  return {
    // map from an array type (the selectedGoals) to a simple boolean type
    hasSelection: state.ui.selection.selectedGoals.length > 0,
    hasHover: state.ui.hover.hoveredGoal && state.ui.hover.hoveredGoal !== state.ui.goalForm.editAddress,
    goalFormIsOpen: state.ui.goalForm.isOpen,
    translate: state.ui.viewport.translate,
    scale: state.ui.viewport.scale
  }
}

export default connect(mapStateToProps)(App)
