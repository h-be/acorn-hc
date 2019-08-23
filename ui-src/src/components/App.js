import React from 'react'
import PropTypes from 'prop-types'
import { connect } from 'react-redux'

import GoalForm from './GoalForm'
import Help from './Help'
import MultiEditBar from './MultiEditBar'
import HoverOverlay from './HoverOverlay'

function App(props) {
  const { hasSelection, hasHover, goalFormIsOpen } = props
  return (
    <div>
      {goalFormIsOpen && <GoalForm />}
      <Help />
      {hasSelection && <MultiEditBar />}
      {hasHover && <HoverOverlay />}
    </div>
  )
}

App.propTypes = {
  hasSelection: PropTypes.bool.isRequired, // whether or not there are Goals selected
  hasHover: PropTypes.bool.isRequired, // whether or not a Goal is hovered over
  goalFormIsOpen: PropTypes.bool.isRequired
}

function mapStateToProps(state) {
  return {
    // map from an array type (the selectedGoals) to a simple boolean type
    hasSelection: state.ui.selection.selectedGoals.length > 0,
    hasHover: state.ui.hover.hoveredGoal && state.ui.hover.hoveredGoal !== state.ui.goalForm.editAddress,
    goalFormIsOpen: state.ui.goalForm.isOpen
  }
}

export default connect(mapStateToProps)(App)
