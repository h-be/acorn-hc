import React from 'react'
import { connect } from 'react-redux'
import PropTypes from 'prop-types'

import Icon from './Icon'

import {
  archiveGoal
} from '../goals/actions'
import {
  closeGoalForm
} from '../goal-form/actions'

function VerticalActionsList(props) {
    const list = [
      ['squirrel.png', 'squirrels', 'onSquirrelsClick'],
      ['archive.svg', 'archive', 'onArchiveClick']
    ]

    return (
      <div className='vertical_actions_list'>
        {list.map(([icon, text, clickKey], index) => (
          <div className='action_list_item' key={index} onClick={() => props[clickKey](props.goalAddress)}>
            <Icon name={icon} />
            <span>{text}</span>
          </div>))}
      </div>
    )
}

VerticalActionsList.propTypes = {
  goalAddress: PropTypes.string.isRequired,
  onSquirrelsClick: PropTypes.func.isRequired,
  onArchiveClick: PropTypes.func.isRequired
}

function mapStateToProps(state) {
  return {
    goalAddress: state.ui.goalForm.editAddress
  }
}

function mapDispatchToProps(dispatch) {
  return {
    onSquirrelsClick: (address) => {},
    onArchiveClick: (address) => {
      dispatch(archiveGoal.create({ address }))
    }
  }
}

export default connect(mapStateToProps, mapDispatchToProps)(VerticalActionsList)
