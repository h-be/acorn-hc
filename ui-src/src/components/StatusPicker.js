import React from 'react'
import PropTypes from 'prop-types'

import StatusIcon from './StatusIcon'

function StatusPicker({ selectedStatus, statusClicked }) {

    const statuses = [
        'Uncertain',
        'Incomplete',
        'Complete',
        'InReview'
    ]

    return (
        <div className='status_picker vertical_action_overlay'>
            {statuses.map(status => (
                <StatusIcon size='small' status={status} selected={selectedStatus === status} onClick={statusClicked} />
            ))}
        </div>
    )
}

StatusPicker.propTypes = {
    selectedStatus: PropTypes.string.isRequired,
    statusClicked: PropTypes.func.isRequired
}

export default StatusPicker