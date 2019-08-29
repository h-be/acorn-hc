import React, { useState } from 'react'
import { connect } from 'react-redux'
import PropTypes from 'prop-types'

import Icon from './Icon'
import PeoplePicker from './PeoplePicker'
import StatusPicker from './StatusPicker'
import StatusIcon from './StatusIcon'

import {
  archiveGoal
} from '../goals/actions'
import {
  closeGoalForm
} from '../goal-form/actions'

function VerticalActionsList(props) {

    const defaultViews = {
      status: false,
      squirrels: false
    }
    const [viewsOpen, setViews] = useState(defaultViews)

    return (
      <div className='vertical_actions_list'>
        {/* status */}
        <div className='action_list_item' key='status' onClick={() => setViews({ ...defaultViews, status: !viewsOpen.status })}>
          <StatusIcon status={props.goal.status} />
          <span>status</span>
        </div>
        {viewsOpen.status && <StatusPicker />}
        {/* squirrels */}
        <div className='action_list_item' key='squirrels' onClick={() => setViews({ ...defaultViews, squirrels: !viewsOpen.squirrels })}>
          <Icon name='squirrel.png' />
          <span>squirrels</span>
        </div>
        {viewsOpen.squirrels && <PeoplePicker />}
        {/* archive */}
        <div className='action_list_item' key='archive' onClick={() => props.onArchiveClick(props.goalAddress)}>
          <Icon name='archive.svg' />
          <span>archive</span>
        </div>
      </div>
    )
}

VerticalActionsList.propTypes = {
  goalAddress: PropTypes.string.isRequired,
  goal: PropTypes.shape({}).isRequired,
  onArchiveClick: PropTypes.func.isRequired
}

function mapStateToProps(state) {
  const goalAddress = state.ui.goalForm.editAddress
  return {
    goalAddress,
    goal: state.goals[goalAddress]
  }
}

function mapDispatchToProps(dispatch) {
  return {
    onArchiveClick: (address) => {
      dispatch(archiveGoal.create({ address }))
    }
  }
}

export default connect(mapStateToProps, mapDispatchToProps)(VerticalActionsList)
