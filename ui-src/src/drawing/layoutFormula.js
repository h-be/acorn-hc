import { goalWidth, goalHeight } from './dimensions'


function findAParent(edges, content) {
  // highly oversimplified
  const edge = edges.find(e => e.child === content)
  return edge ? edge.parent : null
}


function mapGoalToHierarchy(goal, edges) {
  // setup and run a recursive call to find depth/hierarchy in the graph
  // TODO: make this more resilient, highly oversimplified
  let hierarchy = 1

  function checkHierarchy(content) {
    const parent = findAParent(edges, content)
    if (parent) {
      // increment the hierarchy var each time an additional parent
      // is found
      hierarchy++
      checkHierarchy(parent)
    }
  }

  checkHierarchy(goal.content)

  return {
    hierarchy,
    goal
  }
}


function mapHierarchyToPosition({ goal, hierarchy }, withHierarchies, screenWidth) {
  const verticalOffset = 10
  const verticalSpacing = 100

  // FIXME: this needs to be related to the display pixel ratio or something
  const RETINA_HACK_HALFSCREEN = 4

  const sameTier = withHierarchies.filter((wH) => wH.hierarchy === hierarchy)
  const indexInTier = sameTier.map((wH) => wH.goal.content).indexOf(goal.content)

  // default position is horizontal half screen
  const x = (screenWidth / RETINA_HACK_HALFSCREEN) - (goalWidth / 2) + (indexInTier * 10)

  // default position is a function of the hierarchical status of the goal
  const y = verticalOffset + ((goalHeight + verticalSpacing) * hierarchy) + (indexInTier * 10)

  return {
    x,
    y
  }
}

export default function layoutFormula(screenWidth, goals, edges) {
  // assign hierarchical statuses to things
  const withHierarchies = goals.map(g => mapGoalToHierarchy(g, edges))

  // use positions in the hierarchy to determine coordinates
  return withHierarchies.map(wH => mapHierarchyToPosition(wH, withHierarchies, screenWidth))
}
