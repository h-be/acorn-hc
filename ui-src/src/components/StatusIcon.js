import React from 'react'
import PropTypes from 'prop-types'

function StatusIcon({ status, selected, onClick }) {
    const classList = `status_color tooltip_container bg_${status} ${selected ? 'has_status' : ''} ${onClick ? 'can_click' : ''}`

    // change camel case from spaced words e.g. "InReview" to "In Review"
    const readable = status.replace(/([A-Z])/g, ' $1')

    return (
        <div className={classList} onClick={() => onClick(status)}>
            <div className={`tooltip bg_${status}`}>{readable}</div>
        </div>
    )
}

StatusIcon.propTypes = {
    status: PropTypes.string.isRequired,
    selected: PropTypes.bool,
    onClick: PropTypes.func
}

export default StatusIcon